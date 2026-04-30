use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use dawnline::action::Action;
use dawnline::config::Config;
use dawnline::state::AppState;
use dawnline::store::EventStore;
use dawnline::theme::DawnTheme;
use dawnline::ui;
use dawnline::update::{self, Command};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::env;
use std::io;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        return run_cli(args);
    }

    run_tui()
}

fn run_cli(args: Vec<String>) -> Result<()> {
    let mut store = EventStore::open_default()?;
    store.ensure_session()?;

    // this is subject to a refactor later on
    // I don't like --flags being added to commands in a tool like this
    match args.first().map(String::as_str) {
        Some("task") => {
            let priority = args.get(1).map(String::as_str) == Some("--priority");
            let title = if priority {
                args[2..].join(" ")
            } else {
                args[1..].join(" ")
            };
            store.add_task(title, priority)?;
            println!("Task added");
        }
        Some("block") => {
            let (at, title) = if args.get(1).map(String::as_str) == Some("--at") {
                (args.get(2).cloned(), args[3..].join(" "))
            } else {
                (None, args[1..].join(" "))
            };
            store.add_block(title, at)?;
            println!("Block added");
        }
        Some("note") => {
            store.add_note(args[1..].join(" "))?;
            println!("Note added");
        }
        Some("review") => {
            let state = store.load_state()?;
            println!("{}", state.review_text());
        }
        Some("finish") => {
            store.finish_session()?;
            println!("Session finished");
        }
        _ => {
            eprintln!("Usage: dawnline [task|block|note|review|finish]");
        }
    }

    Ok(())
}

fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut store = EventStore::open_default()?;
    store.ensure_session()?;

    let mut app = AppState::from_day(store.load_state()?);
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    let config = Config::load_or_create_default()?;
    let theme = DawnTheme::named(&config.theme.name).with_accent_name(&config.theme.accent);

    loop {
        terminal.draw(|f| ui::draw(f, &app, theme, &config.tagline))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)?
            && let CEvent::Key(key) = event::read()?
        {
            let action = if app.command_mode {
                match key.code {
                    KeyCode::Enter => Action::Enter,
                    KeyCode::Esc => Action::Esc,
                    KeyCode::Backspace => Action::Backspace,
                    KeyCode::Char(c) => Action::Char(c),
                    _ => Action::Tick,
                }
            } else {
                match key.code {
                    KeyCode::Char(':') => Action::OpenCommand,
                    KeyCode::Char('?') => Action::ToggleHelp,
                    KeyCode::Char('q') => Action::Quit,
                    KeyCode::Char(c) => Action::Char(c),
                    KeyCode::Enter => Action::Enter,
                    KeyCode::Esc => Action::Esc,
                    KeyCode::Backspace => Action::Backspace,
                    KeyCode::Up => Action::Up,
                    KeyCode::Down => Action::Down,
                    KeyCode::Left => Action::Left,
                    KeyCode::Right => Action::Right,
                    KeyCode::Tab => Action::NextTab,
                    KeyCode::BackTab => Action::PrevTab,
                    _ => Action::Tick,
                }
            };

            if let Some(command) = update::update(&mut app, action)
                && process_command(command, &mut store, &mut app)?
            {
                return Ok(());
            }
        }

        if last_tick.elapsed() >= tick_rate {
            update::update(&mut app, Action::Tick);
            last_tick = Instant::now();
        }
    }
}

fn process_command(command: Command, store: &mut EventStore, app: &mut AppState) -> Result<bool> {
    match command {
        Command::Quit => return Ok(true),
        Command::AppendEvent(event) => {
            store.append(event)?;
            app.day = store.load_state()?;
            app.sync_selection();
        }
        Command::AppendEvents(events) => {
            for event in events {
                store.append(event)?;
            }
            app.day = store.load_state()?;
            app.sync_selection();
        }
        Command::RunPalette(input) => {
            let message = store.run_palette_command(&input)?;
            app.day = store.load_state()?;
            app.sync_selection();
            app.command_buffer.clear();
            app.command_mode = false;
            app.status_message = message;
        }
    }

    Ok(false)
}

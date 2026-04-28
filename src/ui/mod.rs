use crate::state::{AppState, CurrentScreen};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
};

pub mod execute;
pub mod palette;
pub mod plan;
pub mod review;

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_header(f, app, chunks[0]);

    match app.current_screen {
        CurrentScreen::Plan => plan::render(f, app, chunks[1]),
        CurrentScreen::Execute => execute::render(f, app, chunks[1]),
        CurrentScreen::Review => review::render(f, app, chunks[1]),
    }

    render_footer(f, app, chunks[2]);

    if app.command_mode {
        render_command(f, app);
    }

    if app.show_help {
        render_help(f);
    }
}

fn render_header(f: &mut Frame, app: &AppState, area: Rect) {
    let titles = vec![" PLAN ", " EXECUTE ", " REVIEW "];

    let current_index = match app.current_screen {
        CurrentScreen::Plan => 0,
        CurrentScreen::Execute => 1,
        CurrentScreen::Review => 2,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" DAWNLINE "))
        .select(current_index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );

    f.render_widget(tabs, area);
}

fn render_footer(f: &mut Frame, app: &AppState, area: Rect) {
    let line = Line::from(vec![
        Span::styled("enter", Style::default().fg(Color::Cyan)),
        Span::raw(" execute  "),
        Span::styled(":", Style::default().fg(Color::Cyan)),
        Span::raw(" command  "),
        Span::styled("space", Style::default().fg(Color::Cyan)),
        Span::raw(" done  "),
        Span::styled("s", Style::default().fg(Color::Cyan)),
        Span::raw(" start  "),
        Span::styled("d", Style::default().fg(Color::Cyan)),
        Span::raw(" drop  "),
        Span::styled("f", Style::default().fg(Color::Cyan)),
        Span::raw(" finish  "),
        Span::styled("?", Style::default().fg(Color::Cyan)),
        Span::raw(" help  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(" quit  "),
        Span::styled("| ", Style::default().fg(Color::DarkGray)),
        Span::raw(app.status_message.clone()),
    ]);

    let footer = Paragraph::new(line).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}

fn render_command(f: &mut Frame, app: &AppState) {
    let area = centered_rect(70, 20, f.area());
    f.render_widget(Clear, area);

    let input = Paragraph::new(Line::from(vec![
        Span::styled(": ", Style::default().fg(Color::Cyan)),
        Span::styled(
            format!("{}█", app.command_buffer),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(" COMMAND "),
    );

    f.render_widget(input, area);
}

fn render_help(f: &mut Frame) {
    let area = centered_rect(60, 55, f.area());
    f.render_widget(Clear, area);

    let lines = vec![
        Line::from("enter     begin execution"),
        Line::from("f         enter review"),
        Line::from(":         command palette"),
        Line::from("space     complete selected task"),
        Line::from("s         start selected block"),
        Line::from("d         drop selected task"),
        Line::from("x         remove selected task"),
        Line::from("?         toggle help"),
        Line::from("1         quit"),
    ];

    let help = Paragraph::new(lines)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" HELP "));

    f.render_widget(help, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

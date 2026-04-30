use crate::action::Action;
use crate::models::{Event, EventKind};
use crate::state::{ActivePane, AppState, CurrentScreen};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    AppendEvent(Event),
    AppendEvents(Vec<Event>),
    RunPalette(String),
}

pub fn update(app: &mut AppState, action: Action) -> Option<Command> {
    if app.command_mode {
        return handle_command_mode(app, action);
    }

    match action {
        Action::Quit => return Some(Command::Quit),
        Action::Tick => {}
        Action::Resize(_, _) => {}

        Action::NextTab | Action::PrevTab => app.next_pane(),
        Action::ToggleHelp => app.show_help = !app.show_help,

        Action::OpenCommand => {
            app.command_mode = true;
            app.command_buffer.clear();
        }
        Action::Enter => app.start_execution(),
        Action::Esc => {
            app.show_help = false;
            app.command_mode = false;
        }
        Action::Backspace => {}
        Action::Up => app.scroll_up(),
        Action::Down => app.scroll_down(),
        Action::Left | Action::Right => app.flow_hint(),

        Action::Char(c) => return handle_char_input(app, c),

        Action::SubmitCommand => {
            return Some(Command::RunPalette(app.command_buffer.clone()));
        }
        Action::StartSelectedBlock => {
            if app.active_pane != ActivePane::Timeline {
                app.status_message = "select timeline to start a block".to_string();
                return None;
            }

            if let Some(block_id) = app.selected_block_id() {
                let mut events = Vec::new();

                if let Some(active_id) = app.day.active_block().map(|block| block.id)
                    && active_id != block_id
                {
                    events.push(Event::new(EventKind::BlockFinished {
                        block_id: active_id,
                    }));
                }

                events.push(Event::new(EventKind::BlockStarted { block_id }));

                return Some(Command::AppendEvents(events));
            }

            app.status_message = "no block selected".to_string();
        }
        Action::CompleteSelectedTask => {
            if app.active_pane != ActivePane::Tasks {
                app.status_message = "select tasks to complete a task".to_string();
                return None;
            }

            if let Some(task_id) = app.selected_task_id() {
                return Some(Command::AppendEvent(Event::new(EventKind::TaskCompleted {
                    task_id,
                })));
            }

            app.status_message = "no task selected".to_string();
        }
        Action::DropSelectedTask => {
            if !can_modify_selected_task(app) {
                app.status_message = "select tasks to drop a task".to_string();
                return None;
            }

            if let Some(task_id) = app.selected_task_id() {
                return Some(Command::AppendEvent(Event::new(EventKind::TaskDropped {
                    task_id,
                })));
            }

            app.status_message = "no task selected".to_string();
        }
        Action::RemoveSelectedTask => {
            if !can_modify_selected_task(app) {
                app.status_message = "select tasks to remove a task".to_string();
                return None;
            }

            if let Some(task_id) = app.selected_task_id() {
                return Some(Command::AppendEvent(Event::new(EventKind::TaskRemoved {
                    task_id,
                })));
            }

            app.status_message = "no task selected".to_string();
        }
        Action::StartNewSession => {
            if app.current_screen != CurrentScreen::Review {
                app.flow_hint();
                return None;
            }

            if app.day.day_finished {
                app.status_message = "day already finished".to_string();
                return None;
            }

            app.current_screen = CurrentScreen::Plan;
            app.active_pane = ActivePane::Timeline;
            app.status_message = "new session started. plan the next block".to_string();

            return Some(Command::AppendEvent(Event::new(
                EventKind::SessionStarted {
                    session_id: Uuid::now_v7(),
                },
            )));
        }
        Action::FinishSession => {
            if app.current_screen != CurrentScreen::Execute {
                app.status_message = "execution needs to start to be finished".to_string();
                return None;
            }

            app.current_screen = CurrentScreen::Review;
            app.active_pane = ActivePane::CarryForward;
            app.status_message = "review - finish with a clear record".to_string();

            let mut events = Vec::new();

            if let Some(block_id) = app.day.active_block().map(|block| block.id) {
                events.push(Event::new(EventKind::BlockFinished { block_id }));
            }

            if let Some(session_id) = app.day.current_session_id {
                events.push(Event::new(EventKind::SessionFinished { session_id }));
            }

            if events.is_empty() {
                return None;
            }

            return Some(Command::AppendEvents(events));
        }
        Action::FinishDay => {
            if app.current_screen != CurrentScreen::Review {
                app.flow_hint();
                return None;
            }

            app.status_message = "day finished".to_string();
            return Some(Command::AppendEvent(Event::new(EventKind::DayFinished)));
        }
        Action::ReceiveStoreResult(result) => match result {
            Ok(message) => app.status_message = message,
            Err(error) => app.status_message = format!("Error: {}", error),
        },
    }

    None
}

fn handle_command_mode(app: &mut AppState, action: Action) -> Option<Command> {
    match action {
        Action::Enter | Action::SubmitCommand => {
            let input = app.command_buffer.trim().to_string();

            if input == "execute" || input == "start" {
                app.command_mode = false;
                app.command_buffer.clear();
                app.start_execution();
                return None;
            }

            if input == "review" {
                app.command_mode = false;
                app.command_buffer.clear();
                app.flow_hint();
                return None;
            }

            if let Some(minutes) = focus_minutes_from_command(&input) {
                app.command_mode = false;
                app.command_buffer.clear();

                if app.current_screen != CurrentScreen::Execute {
                    app.status_message = "focus starts in execute".to_string();
                    return None;
                }

                app.start_focus(minutes);
                return Some(Command::AppendEvent(Event::new(EventKind::FocusLogged {
                    minutes,
                })));
            }

            if input == "new session" || input == "session new" {
                app.command_mode = false;
                app.command_buffer.clear();
                return update(app, Action::StartNewSession);
            }

            if input == "finish" {
                app.command_mode = false;
                app.command_buffer.clear();

                return match app.current_screen {
                    CurrentScreen::Execute => update(app, Action::FinishSession),
                    CurrentScreen::Review => update(app, Action::FinishDay),
                    CurrentScreen::Plan => {
                        app.flow_hint();
                        None
                    }
                };
            }

            return Some(Command::RunPalette(input));
        }
        Action::Esc => {
            app.command_mode = false;
            app.command_buffer.clear();
        }
        Action::Backspace => {
            app.command_buffer.pop();
        }
        Action::Char(c) => {
            app.command_buffer.push(c);
        }
        _ => {}
    }

    None
}

fn handle_char_input(app: &mut AppState, c: char) -> Option<Command> {
    match c {
        'q' => return Some(Command::Quit),
        'a' | ':' => {
            app.command_mode = true;
            app.command_buffer.clear();
        }
        'e' => app.start_execution(),
        'p' | 'r' => app.flow_hint(),
        't' => {
            if app.current_screen == CurrentScreen::Execute {
                app.start_focus(45);
                return Some(Command::AppendEvent(Event::new(EventKind::FocusLogged {
                    minutes: 45,
                })));
            }

            app.status_message = "focus starts in execute".to_string();
        }
        'n' => return update(app, Action::StartNewSession),
        'f' => {
            return match app.current_screen {
                CurrentScreen::Execute => update(app, Action::FinishSession),
                CurrentScreen::Review => update(app, Action::FinishDay),
                CurrentScreen::Plan => {
                    app.flow_hint();
                    None
                }
            };
        }
        ' ' => {
            if app.current_screen != CurrentScreen::Review {
                return update(app, Action::CompleteSelectedTask);
            }
        }
        's' => {
            if app.current_screen != CurrentScreen::Review {
                return update(app, Action::StartSelectedBlock);
            }
        }
        'd' => {
            return update(app, Action::DropSelectedTask);
        }
        'x' => {
            return update(app, Action::RemoveSelectedTask);
        }
        _ => {}
    }

    None
}

fn focus_minutes_from_command(input: &str) -> Option<u32> {
    let input = input.trim().trim_start_matches(':').trim();

    if input == "focus" {
        return Some(45);
    }

    input
        .strip_prefix("focus ")
        .map(|minutes| minutes.parse::<u32>().unwrap_or(45))
}

fn can_modify_selected_task(app: &AppState) -> bool {
    app.active_pane == ActivePane::Tasks
        || (app.current_screen == CurrentScreen::Review
            && app.active_pane == ActivePane::CarryForward)
}

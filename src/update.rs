use crate::action::Action;
use crate::models::{Event, EventKind};
use crate::state::{AppState, CurrentScreen};

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    AppendEvent(Event),
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

        Action::NextTab | Action::PrevTab => app.flow_hint(),
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
            if let Some(block_id) = app.selected_block_id() {
                return Some(Command::AppendEvent(Event::new(EventKind::BlockStarted {
                    block_id,
                })));
            }
        }
        Action::CompleteSelectedTask => {
            if let Some(task_id) = app.selected_task_id() {
                return Some(Command::AppendEvent(Event::new(EventKind::TaskCompleted {
                    task_id,
                })));
            }
        }
        Action::DropSelectedTask => {
            if let Some(task_id) = app.selected_task_id() {
                return Some(Command::AppendEvent(Event::new(EventKind::TaskDropped {
                    task_id,
                })));
            }
        }
        Action::RemoveSelectedTask => {
            if let Some(task_id) = app.selected_task_id() {
                return Some(Command::AppendEvent(Event::new(EventKind::TaskRemoved {
                    task_id,
                })));
            }
        }
        Action::FinishSession => {
            if app.current_screen != CurrentScreen::Execute {
                app.status_message = "execution needs to start to be finished".to_string();
                return None;
            }

            app.current_screen = CurrentScreen::Review;
            app.status_message = "review - finish with a clear record".to_string();

            if let Some(session_id) = app.day.current_session_id {
                return Some(Command::AppendEvent(Event::new(
                    EventKind::SessionFinished { session_id },
                )));
            }
        }
        Action::FinishDay => {
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

            if input == "finish" && app.current_screen == CurrentScreen::Execute {
                app.current_screen = CurrentScreen::Review;
                app.status_message = "review. finish with a clear record".to_string();
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
        'f' => return update(app, Action::FinishSession),
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
            if app.current_screen != CurrentScreen::Review {
                return update(app, Action::DropSelectedTask);
            }
        }
        'x' => {
            if app.current_screen != CurrentScreen::Review {
                return update(app, Action::RemoveSelectedTask);
            }
        }
        _ => {}
    }

    None
}

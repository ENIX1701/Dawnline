use crate::models::{Block, DayState, Task};
use ratatui::widgets::ListState;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    Execute,
    Plan,
    Review,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_screen: CurrentScreen,
    pub day: DayState,
    pub task_state: ListState,
    pub block_state: ListState,
    pub command_mode: bool,
    pub command_buffer: String,
    pub show_help: bool,
    pub status_message: String,
}

impl AppState {
    pub fn from_day(day: DayState) -> Self {
        let mut task_state = ListState::default();
        if !day.active_tasks().is_empty() {
            task_state.select(Some(0));
        }

        let mut block_state = ListState::default();
        if !day.blocks.is_empty() {
            block_state.select(Some(0));
        }

        Self {
            current_screen: CurrentScreen::Plan,
            day,
            task_state,
            block_state,
            command_mode: false,
            command_buffer: String::new(),
            show_help: false,
            status_message: "READY - press '?' for help".to_string(),
        }
    }

    pub fn next_tab(&mut self) {
        self.current_screen = match self.current_screen {
            CurrentScreen::Plan => CurrentScreen::Execute,
            CurrentScreen::Execute => CurrentScreen::Review,
            CurrentScreen::Review => CurrentScreen::Plan,
        }
    }

    pub fn prev_tab(&mut self) {
        self.current_screen = match self.current_screen {
            CurrentScreen::Plan => CurrentScreen::Review,
            CurrentScreen::Execute => CurrentScreen::Plan,
            CurrentScreen::Review => CurrentScreen::Execute,
        }
    }

    pub fn visible_blocks(&self) -> Vec<&Block> {
        self.day.blocks.iter().collect()
    }

    pub fn visible_tasks(&self) -> Vec<&Task> {
        self.day.active_tasks()
    }

    pub fn selected_task_id(&self) -> Option<Uuid> {
        let tasks = self.visible_tasks();
        self.task_state
            .selected()
            .and_then(|i| tasks.get(i))
            .map(|task| task.id)
    }

    pub fn selected_block_id(&self) -> Option<Uuid> {
        let blocks = self.visible_blocks();
        self.block_state
            .selected()
            .and_then(|i| blocks.get(i))
            .map(|block| block.id)
    }

    pub fn scroll_down(&mut self) {
        match self.current_screen {
            CurrentScreen::Plan | CurrentScreen::Execute => {
                let len = self.visible_tasks().len();
                select_next(&mut self.task_state, len);
            }
            CurrentScreen::Review => {}
        }
    }

    pub fn scroll_up(&mut self) {
        match self.current_screen {
            CurrentScreen::Plan | CurrentScreen::Execute => {
                let len = self.visible_tasks().len();
                select_prev(&mut self.task_state, len);
            }
            CurrentScreen::Review => {}
        }
    }

    pub fn start_execution(&mut self) {
        match self.current_screen {
            CurrentScreen::Plan => {
                self.current_screen = CurrentScreen::Execute;
                self.status_message = "execute. time to get things done.".to_string();
            }
            CurrentScreen::Execute => {
                self.status_message = "getting things done. f to finish".to_string();
            }
            CurrentScreen::Review => {
                self.status_message = "review. see what worked. see what didnt'.".to_string();
            }
        }
    }

    pub fn flow_hint(&mut self) {
        self.status_message = match self.current_screen {
            CurrentScreen::Plan => "enter to begin executing".to_string(),
            CurrentScreen::Execute => "f or :finish to review".to_string(),
            CurrentScreen::Review => "review done. start a new session later.".to_string(),
        };
    }
}

fn select_next(state: &mut ListState, len: usize) {
    if len == 0 {
        state.select(None);
        return;
    }

    let i = match state.selected() {
        Some(i) => {
            if i >= len - 1 {
                i
            } else {
                i + 1
            }
        }
        None => 0,
    };

    state.select(Some(i));
}

fn select_prev(state: &mut ListState, len: usize) {
    if len == 0 {
        state.select(None);
        return;
    }

    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                i
            } else {
                i - 1
            }
        }
        None => 0,
    };

    state.select(Some(i));
}

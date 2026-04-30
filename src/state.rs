use crate::models::{Block, BlockTiming, DayState, Task};
use chrono::{DateTime, Local};
use ratatui::widgets::ListState;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    Plan,
    Execute,
    Review,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    Timeline,
    Tasks,
    CarryForward,
    Session,
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
    pub active_pane: ActivePane,
    pub focus_started_at: Option<DateTime<Local>>,
    pub focus_minutes: u32,
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
            active_pane: ActivePane::Timeline,
            focus_started_at: None,
            focus_minutes: 45,
        }
    }

    pub fn next_pane(&mut self) {
        if self.current_screen == CurrentScreen::Review {
            self.active_pane = match self.active_pane {
                ActivePane::CarryForward => ActivePane::Session,
                ActivePane::Session | ActivePane::Timeline | ActivePane::Tasks => {
                    ActivePane::CarryForward
                }
            };

            self.status_message = match self.active_pane {
                ActivePane::CarryForward => "carry forward selected".to_string(),
                ActivePane::Session => "session summary selected".to_string(),
                ActivePane::Timeline | ActivePane::Tasks => {
                    unreachable!("review pane should be carry-forward or session")
                }
            };

            return;
        }

        self.active_pane = match self.active_pane {
            ActivePane::Timeline => ActivePane::Tasks,
            ActivePane::Tasks | ActivePane::CarryForward | ActivePane::Session => {
                ActivePane::Timeline
            }
        };

        self.status_message = match self.active_pane {
            ActivePane::Timeline => "timeline selected".to_string(),
            ActivePane::Tasks => "tasks selected".to_string(),
            ActivePane::CarryForward | ActivePane::Session => {
                unreachable!("planning pane should be timeline or tasks")
            }
        };
    }

    pub fn visible_blocks(&self) -> Vec<&Block> {
        let mut blocks: Vec<&Block> = self.day.blocks.iter().collect();

        blocks.sort_by(|a, b| {
            block_group_key(a)
                .cmp(&block_group_key(b))
                .then_with(|| a.created_at.cmp(&b.created_at))
        });

        blocks
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
        if self.current_screen == CurrentScreen::Review
            && self.active_pane != ActivePane::CarryForward
        {
            return;
        }

        match self.active_pane {
            ActivePane::Timeline => {
                let len = self.visible_blocks().len();
                select_next(&mut self.block_state, len);
            }
            ActivePane::Tasks => {
                let len = self.visible_tasks().len();
                select_next(&mut self.task_state, len);
            }
            ActivePane::CarryForward => {
                let len = self.visible_tasks().len();
                select_next(&mut self.task_state, len);
            }
            ActivePane::Session => {}
        }
    }

    pub fn scroll_up(&mut self) {
        if self.current_screen == CurrentScreen::Review
            && self.active_pane != ActivePane::CarryForward
        {
            return;
        }

        match self.active_pane {
            ActivePane::Timeline => {
                let len = self.visible_blocks().len();
                select_prev(&mut self.block_state, len);
            }
            ActivePane::Tasks => {
                let len = self.visible_tasks().len();
                select_prev(&mut self.task_state, len);
            }
            ActivePane::CarryForward => {
                let len = self.visible_tasks().len();
                select_prev(&mut self.task_state, len);
            }
            ActivePane::Session => {}
        }
    }

    pub fn start_execution(&mut self) {
        match self.current_screen {
            CurrentScreen::Plan => {
                self.current_screen = CurrentScreen::Execute;
                self.active_pane = ActivePane::Tasks;
                self.status_message = "execute. time to get things done.".to_string();
            }
            CurrentScreen::Execute => {
                self.status_message = "getting things done. f to finish".to_string();
            }
            CurrentScreen::Review => {
                self.active_pane = ActivePane::CarryForward;
                self.status_message = "review. see what worked. see what didnt'.".to_string();
            }
        }
    }

    pub fn flow_hint(&mut self) {
        self.status_message = match self.current_screen {
            CurrentScreen::Plan => "enter to begin executing".to_string(),
            CurrentScreen::Execute => "f or :finish to review".to_string(),
            CurrentScreen::Review => "n new session, f finish day".to_string(),
        };
    }

    pub fn sync_selection(&mut self) {
        let block_len = self.visible_blocks().len();
        let task_len = self.visible_tasks().len();

        clamp_selection(&mut self.block_state, block_len);
        clamp_selection(&mut self.task_state, task_len);
    }

    pub fn start_focus(&mut self, minutes: u32) {
        self.focus_started_at = Some(Local::now());
        self.focus_minutes = minutes;
        self.status_message = format!("focus started: {}m", minutes);
    }

    pub fn focus_remaining_seconds(&self) -> Option<i64> {
        let started = self.focus_started_at?;
        let elapsed = Local::now().signed_duration_since(started).num_seconds();
        let total = i64::from(self.focus_minutes) * 60;

        Some((total - elapsed).max(0))
    }
}

fn clamp_selection(state: &mut ListState, len: usize) {
    if len == 0 {
        state.select(None);
        return;
    }

    let selected = state.selected().unwrap_or(0);
    state.select(Some(selected.min(len - 1)));
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

fn block_group_key(block: &Block) -> (u8, u32, String) {
    match &block.timing {
        BlockTiming::Loose { label } => match label.to_ascii_lowercase().as_str() {
            "now" => (0, 0, String::new()),
            "next" => (1, 0, String::new()),
            "later" => (3, 0, String::new()),
            other => (4, 0, other.to_string()),
        },
        BlockTiming::Exact { start, .. } => {
            (2, time_minutes(start).unwrap_or(u32::MAX), String::new())
        }
    }
}

fn time_minutes(value: &str) -> Option<u32> {
    let (hour, minute) = value.split_once(':')?;
    let hour = hour.parse::<u32>().ok()?;
    let minute = minute.parse::<u32>().ok()?;

    if hour < 24 && minute < 60 {
        Some(hour * 60 + minute)
    } else {
        None
    }
}

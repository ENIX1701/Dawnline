use EventKind::*;
use chrono::{DateTime, Local, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Open,
    Done,
    Dropped,
    Removed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Open => write!(f, "open"),
            TaskStatus::Done => write!(f, "done"),
            TaskStatus::Dropped => write!(f, "dropped"),
            TaskStatus::Removed => write!(f, "removed"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockStatus {
    Planned,
    Active,
    Done,
}

impl fmt::Display for BlockStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockStatus::Planned => write!(f, "planned"),
            BlockStatus::Active => write!(f, "active"),
            BlockStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum BlockTiming {
    Exact { start: String, end: Option<String> },
    Loose { label: String },
}

impl fmt::Display for BlockTiming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockTiming::Exact { start, end } => {
                if let Some(end) = end {
                    write!(f, "{}-{}", start, end)
                } else {
                    write!(f, "{}", start)
                }
            }
            BlockTiming::Loose { label } => write!(f, "{}", label),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub priority: bool,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub title: String,
    pub intent: Option<String>,
    pub timing: BlockTiming,
    pub status: BlockStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub at: DateTime<Utc>,
    pub kind: EventKind,
}

impl Event {
    pub fn new(kind: EventKind) -> Self {
        Self {
            id: Uuid::now_v7(),
            at: Utc::now(),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EventKind {
    SessionStarted {
        session_id: Uuid,
    },
    SessionFinished {
        session_id: Uuid,
    },
    DayFinished,

    BlockAdded {
        block_id: Uuid,
        title: String,
        intent: Option<String>,
        timing: BlockTiming,
    },
    BlockStarted {
        block_id: Uuid,
    },
    BlockFinished {
        block_id: Uuid,
    },

    TaskAdded {
        task_id: Uuid,
        title: String,
        priority: bool,
    },
    TaskCompleted {
        task_id: Uuid,
    },
    TaskDropped {
        task_id: Uuid,
    },
    TaskRemoved {
        task_id: Uuid,
    },
    TaskPriorityChanged {
        task_id: Uuid,
        priority: bool,
    },

    NoteAdded {
        note_id: Uuid,
        text: String,
    },
    FocusLogged {
        minutes: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayState {
    pub date: NaiveDate,
    pub day_finished: bool,
    pub current_session_id: Option<Uuid>,
    pub sessions: Vec<Session>,
    pub blocks: Vec<Block>,
    pub tasks: Vec<Task>,
    pub notes: Vec<Note>,
    pub focus_minutes: u32,
    pub history: Vec<Event>,
}

impl Default for DayState {
    fn default() -> Self {
        Self {
            date: Local::now().date_naive(),
            day_finished: false,
            current_session_id: None,
            sessions: Vec::new(),
            blocks: Vec::new(),
            tasks: Vec::new(),
            notes: Vec::new(),
            focus_minutes: 0,
            history: Vec::new(),
        }
    }
}

impl DayState {
    pub fn replay(events: &[Event]) -> Self {
        let mut state = DayState::default();

        for event in events {
            state.apply(event.clone());
        }

        state
    }

    pub fn apply(&mut self, event: Event) {
        let at = event.at;

        match &event.kind {
            SessionStarted { session_id } => {
                self.current_session_id = Some(*session_id);
                self.sessions.push(Session {
                    id: *session_id,
                    started_at: at,
                    finished_at: None,
                });
            }
            SessionFinished { session_id } => {
                if let Some(session) = self.sessions.iter_mut().find(|s| s.id == *session_id) {
                    session.finished_at = Some(at);
                }

                if self.current_session_id == Some(*session_id) {
                    self.current_session_id = None;
                }
            }
            DayFinished => {
                self.day_finished = true;

                for session in &mut self.sessions {
                    if session.finished_at.is_none() {
                        session.finished_at = Some(at);
                    }
                }

                self.current_session_id = None;
            }
            BlockAdded {
                block_id,
                title,
                intent,
                timing,
            } => {
                self.blocks.push(Block {
                    id: *block_id,
                    title: title.clone(),
                    intent: intent.clone(),
                    timing: timing.clone(),
                    status: BlockStatus::Planned,
                    created_at: at,
                    started_at: None,
                    finished_at: None,
                });
            }
            BlockStarted { block_id } => {
                for block in &mut self.blocks {
                    if block.status == BlockStatus::Active {
                        block.status = BlockStatus::Planned;
                    }

                    if block.id == *block_id {
                        block.status = BlockStatus::Active;
                        block.started_at = Some(at);
                    }
                }
            }
            BlockFinished { block_id } => {
                if let Some(block) = self.blocks.iter_mut().find(|b| b.id == *block_id) {
                    block.status = BlockStatus::Done;
                    block.finished_at = Some(at);
                }
            }
            TaskAdded {
                task_id,
                title,
                priority,
            } => {
                self.tasks.push(Task {
                    id: *task_id,
                    title: title.clone(),
                    priority: *priority,
                    status: TaskStatus::Open,
                    created_at: at,
                    closed_at: None,
                });
            }
            TaskCompleted { task_id } => {
                self.close_task(*task_id, TaskStatus::Done, at);
            }
            TaskDropped { task_id } => {
                self.close_task(*task_id, TaskStatus::Dropped, at);
            }
            TaskRemoved { task_id } => {
                self.close_task(*task_id, TaskStatus::Removed, at);
            }
            TaskPriorityChanged { task_id, priority } => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == *task_id) {
                    task.priority = *priority;
                }
            }
            NoteAdded { note_id, text } => {
                self.notes.push(Note {
                    id: *note_id,
                    text: text.clone(),
                    created_at: at,
                });
            }
            FocusLogged { minutes } => {
                self.focus_minutes += *minutes;
            }
        }

        self.history.push(event);
    }

    pub fn close_task(&mut self, task_id: Uuid, status: TaskStatus, at: DateTime<Utc>) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = status;
            task.closed_at = Some(at);
        }
    }

    pub fn active_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Open)
            .collect();

        tasks.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.created_at.cmp(&b.created_at))
        });

        tasks
    }

    pub fn completed_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Done)
            .collect()
    }

    pub fn dropped_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Dropped)
            .collect()
    }

    pub fn priority_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Open && t.priority)
            .collect()
    }

    pub fn active_block(&self) -> Option<&Block> {
        self.blocks.iter().find(|b| b.status == BlockStatus::Active)
    }

    pub fn completed_blocks(&self) -> Vec<&Block> {
        self.blocks
            .iter()
            .filter(|b| b.status == BlockStatus::Done)
            .collect()
    }

    pub fn review_text(&self) -> String {
        format!(
            "DAWNLINE REVIEW\n\nCompleted tasks: {}\nOpen priority tasks: {}\nCompleted blocks: {}\nDropped tasks: {}\nFocus time: {}m\n",
            self.completed_tasks().len(),
            self.priority_tasks().len(),
            self.completed_blocks().len(),
            self.dropped_tasks().len(),
            self.focus_minutes
        )
    }
}

use crate::models::{BlockTiming, DayState, Event, EventKind};
use color_eyre::Result;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EventStore {
    pub path: PathBuf,
}

impl EventStore {
    pub fn open_default() -> Result<Self> {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let dir = PathBuf::from(home).join(".local/share/dawnline"); // linux only for now... x3
        fs::create_dir_all(&dir)?;

        Ok(Self {
            path: dir.join("events.jsonl"),
        })
    }

    pub fn load_events(&self) -> Result<Vec<Event>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut events = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            events.push(serde_json::from_str::<Event>(&line)?);
        }

        Ok(events)
    }

    pub fn load_state(&self) -> Result<DayState> {
        let events = self.load_events()?;
        Ok(DayState::replay(&events))
    }

    pub fn append(&mut self, event: Event) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let line = serde_json::to_string(&event)?;
        writeln!(file, "{}", line)?;

        Ok(())
    }

    pub fn ensure_session(&mut self) -> Result<()> {
        let state = self.load_state()?;

        if state.current_session_id.is_none() {
            self.append(Event::new(EventKind::SessionStarted {
                session_id: Uuid::now_v7(),
            }))?;
        }

        Ok(())
    }

    pub fn add_task(&mut self, title: String, priority: bool) -> Result<()> {
        if title.trim().is_empty() {
            return Ok(());
        }

        self.append(Event::new(EventKind::TaskAdded {
            task_id: Uuid::now_v7(),
            title,
            priority,
        }))
    }

    pub fn add_block(&mut self, title: String, at: Option<String>) -> Result<()> {
        if title.trim().is_empty() {
            return Ok(());
        }

        let timing = match at {
            Some(start) => BlockTiming::Exact { start, end: None },
            None => BlockTiming::Loose {
                label: "Later".to_string(),
            },
        };

        self.append(Event::new(EventKind::BlockAdded {
            block_id: Uuid::now_v7(),
            title,
            intent: None,
            timing,
        }))
    }

    fn add_block_from_palette(&mut self, rest: &str) -> Result<()> {
        let parts: Vec<&str> = rest.split_whitespace().collect();

        if parts.is_empty() {
            return Ok(());
        }

        let first = parts[0];

        if let Some(label) = loose_label(first) {
            let title = parts[1..].join(" ");

            if title.trim().is_empty() {
                return Ok(());
            }

            return self.append(Event::new(EventKind::BlockAdded {
                block_id: Uuid::now_v7(),
                title,
                intent: None,
                timing: BlockTiming::Loose { label },
            }));
        }

        if looks_like_time(first) {
            let second_is_time = parts.get(1).is_some_and(|value| looks_like_time(value));

            let end = if second_is_time {
                parts.get(1).map(|value| value.to_string())
            } else {
                None
            };

            let title_start = if second_is_time { 2 } else { 1 };
            let title = parts[title_start..].join(" ");

            if title.trim().is_empty() {
                return Ok(());
            }

            return self.append(Event::new(EventKind::BlockAdded {
                block_id: Uuid::now_v7(),
                title,
                intent: None,
                timing: BlockTiming::Exact {
                    start: first.to_string(),
                    end,
                },
            }));
        }

        self.add_block(rest.to_string(), None)
    }

    fn start_block(&mut self, query: &str) -> Result<bool> {
        let state = self.load_state()?;
        let query = query.trim().to_lowercase();

        let block_id = state
            .blocks
            .iter()
            .find(|block| block.title.to_lowercase().contains(&query))
            .map(|block| block.id);

        if let Some(block_id) = block_id {
            self.append(Event::new(EventKind::BlockStarted { block_id }))?;
            return Ok(true);
        }

        Ok(false)
    }

    fn complete_task(&mut self, task_ref: &str) -> Result<bool> {
        if let Some(task_id) = self.find_active_task(task_ref)? {
            self.append(Event::new(EventKind::TaskCompleted { task_id }))?;
            return Ok(true);
        }

        Ok(false)
    }

    fn drop_task(&mut self, task_ref: &str) -> Result<bool> {
        if let Some(task_id) = self.find_active_task(task_ref)? {
            self.append(Event::new(EventKind::TaskDropped { task_id }))?;
            return Ok(true);
        }

        Ok(false)
    }

    fn remove_task(&mut self, task_ref: &str) -> Result<bool> {
        if let Some(task_id) = self.find_active_task(task_ref)? {
            self.append(Event::new(EventKind::TaskRemoved { task_id }))?;
            return Ok(true);
        }

        Ok(false)
    }

    fn find_active_task(&self, task_ref: &str) -> Result<Option<Uuid>> {
        let state = self.load_state()?;
        let tasks = state.active_tasks();
        let task_ref = task_ref.trim();

        if let Ok(index) = task_ref.parse::<usize>() {
            return Ok(tasks.get(index.saturating_sub(1)).map(|task| task.id));
        }

        let query = task_ref.to_lowercase();

        Ok(tasks
            .iter()
            .find(|task| task.title.to_lowercase().contains(&query))
            .map(|task| task.id))
    }

    pub fn add_note(&mut self, text: String) -> Result<()> {
        if text.trim().is_empty() {
            return Ok(());
        }

        self.append(Event::new(EventKind::NoteAdded {
            note_id: Uuid::now_v7(),
            text,
        }))
    }

    pub fn finish_session(&mut self) -> Result<()> {
        let state = self.load_state()?;

        if let Some(session_id) = state.current_session_id {
            self.append(Event::new(EventKind::SessionFinished { session_id }))?;
        }

        Ok(())
    }

    pub fn run_palette_command(&mut self, input: &str) -> Result<String> {
        let input = input.trim();

        if input.is_empty() {
            return Ok("No command".to_string());
        }

        if let Some(title) = input.strip_prefix("add priority task ") {
            self.add_task(title.to_string(), true)?;
            return Ok("Priority task added".to_string());
        }

        if let Some(title) = input.strip_prefix("add task ") {
            self.add_task(title.to_string(), false)?;
            return Ok("Task added".to_string());
        }

        if let Some(rest) = input.strip_prefix("add block ") {
            self.add_block_from_palette(rest)?;

            return Ok("Block added".to_string());
        }

        if let Some(text) = input.strip_prefix("note ") {
            self.add_note(text.to_string())?;
            return Ok("Note added".to_string());
        }

        if let Some(minutes) = input.strip_prefix("focus ") {
            let minutes = minutes.parse::<u32>().unwrap_or(45);
            self.append(Event::new(EventKind::FocusLogged { minutes }))?;
            return Ok(format!("Focus logged: {}m", minutes));
        }

        if let Some(query) = input.strip_prefix("start ") {
            if self.start_block(query)? {
                return Ok("Block started".to_string());
            }

            return Ok(format!("No block matched: {}", query));
        }

        if let Some(task_ref) = input.strip_prefix("drop task ") {
            if self.drop_task(task_ref)? {
                return Ok("Task dropped".to_string());
            }

            return Ok(format!("No active task matched: {}", task_ref));
        }

        if let Some(task_ref) = input.strip_prefix("remove task ") {
            if self.remove_task(task_ref)? {
                return Ok("Task removed".to_string());
            }

            return Ok(format!("No active task matched: {}", task_ref));
        }

        if let Some(task_ref) = input.strip_prefix("done task ") {
            if self.complete_task(task_ref)? {
                return Ok("Task completed".to_string());
            }

            return Ok(format!("No active task matched: {}", task_ref));
        }

        if input == "finish" {
            self.finish_session()?;
            return Ok("Session finished".to_string());
        }

        if input == "review" {
            return Ok("finish execution. then review.".to_string());
        }

        Ok(format!("Unknown command: {}", input))
    }
}

fn looks_like_time(value: &str) -> bool {
    let mut parts = value.split(':');

    let hour = parts.next().unwrap_or_default();
    let minutes = parts.next().unwrap_or_default();

    parts.next().is_none()
        && hour.len() <= 2
        && minutes.len() == 2
        && hour.chars().all(|c| c.is_ascii_digit())
        && minutes.chars().all(|c| c.is_ascii_digit())
}

fn loose_label(value: &str) -> Option<String> {
    match value.to_lowercase().as_str() {
        "now" => Some("now".to_string()),
        "next" => Some("next".to_string()),
        "later" => Some("later".to_string()),
        _ => None,
    }
}

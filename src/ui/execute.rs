use crate::models::{BlockStatus, TaskStatus};
use crate::state::AppState;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_now(f, app, chunks[0]);
    render_tasks(f, app, chunks[1]);
}

fn render_now(f: &mut Frame, app: &AppState, area: Rect) {
    let block_title = app
        .day
        .active_block()
        .map(|block| block.title.clone())
        .unwrap_or_else(|| "No active block".to_string());

    let mut lines = vec![
        Line::from(Span::styled(
            "NOW",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(block_title),
        Line::from(""),
        Line::from(format!("Open tasks: {}", app.day.active_tasks().len())),
        Line::from(format!("Priority: {}", app.day.priority_tasks().len())),
        Line::from(format!("Focus: {}m", app.day.focus_minutes)),
        Line::from(""),
        Line::from(Span::styled(
            "Timeline",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    for block in &app.day.blocks {
        let color = match block.status {
            BlockStatus::Planned => Color::White,
            BlockStatus::Active => Color::Yellow,
            BlockStatus::Done => Color::DarkGray,
        };

        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<8}", block.timing),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(block.title.clone(), Style::default().fg(color)),
        ]));
    }

    let panel = Paragraph::new(lines)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" CURRENT BLOCK "),
        );

    f.render_widget(panel, area);
}

fn render_tasks(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app
        .day
        .active_tasks()
        .iter()
        .map(|task| {
            let marker = if task.priority { "!" } else { " " };

            let task_style = if task.priority {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let status = match task.status {
                TaskStatus::Open => "open",
                TaskStatus::Done => "done",
                TaskStatus::Dropped => "drop",
                TaskStatus::Removed => "rem",
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] ", marker), Style::default().fg(Color::Cyan)),
                Span::styled(task.title.clone(), task_style),
                Span::styled(format!(" {}", status), Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" TASK QUEUE "))
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = app.task_state;
    f.render_stateful_widget(list, area, &mut state);
}

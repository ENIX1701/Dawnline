use crate::models::{BlockStatus, TaskStatus};
use crate::state::AppState;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(35),
            Constraint::Percentage(20),
        ])
        .split(area);

    render_timeline(f, app, chunks[0]);
    render_tasks(f, app, chunks[1]);
    render_detail(f, app, chunks[2]);
}

fn render_timeline(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app
        .day
        .blocks
        .iter()
        .map(|block| {
            let color = match block.status {
                BlockStatus::Planned => Color::White,
                BlockStatus::Active => Color::Yellow,
                BlockStatus::Done => Color::DarkGray,
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{:<8}", block.timing),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(block.title.clone(), Style::default().fg(color)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" TIMELINE "))
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = app.block_state;
    f.render_stateful_widget(list, area, &mut state);
}

fn render_tasks(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app
        .day
        .active_tasks()
        .iter()
        .map(|task| {
            let marker = if task.priority { "!" } else { " " };
            let style = if task.priority {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] ", marker), Style::default().fg(Color::Cyan)),
                Span::styled(task.title.clone(), style),
                Span::styled(
                    format!(" {}", task.status),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" TASKS "))
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = app.task_state;
    f.render_stateful_widget(list, area, &mut state);
}

fn render_detail(f: &mut Frame, app: &AppState, area: Rect) {
    let open = app.day.active_tasks().len();
    let done = app
        .day
        .tasks
        .iter()
        .filter(|task| task.status == TaskStatus::Done)
        .count();

    let lines = vec![
        Line::from(Span::styled(
            "Day",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.day.date.to_string()),
        Line::from(""),
        Line::from(format!("Blocks: {}", app.day.blocks.len())),
        Line::from(format!("Open: {}", open)),
        Line::from(format!("Done: {}", done)),
        Line::from(format!("Notes: {}", app.day.notes.len())),
        Line::from(""),
        Line::from("Commands"),
        Line::from("add task ..."),
        Line::from("add block ..."),
        Line::from("note ..."),
    ];

    let panel = Paragraph::new(lines)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" DAY "));

    f.render_widget(panel, area);
}

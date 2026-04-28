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
            Constraint::Percentage(40),
            Constraint::Percentage(35),
            Constraint::Percentage(25),
        ])
        .split(area);

    render_completed(f, app, chunks[0]);
    render_carry_forward(f, app, chunks[1]);
    render_summary(f, app, chunks[2]);
}

fn render_completed(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app
        .day
        .tasks
        .iter()
        .filter(|task| task.status == TaskStatus::Done)
        .map(|task| {
            ListItem::new(Line::from(vec![
                Span::styled("[x] ", Style::default().fg(Color::Green)),
                Span::raw(task.title.clone()),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" COMPLETED "))
        .highlight_symbol(">> ");

    f.render_widget(list, area);
}

fn render_carry_forward(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app
        .day
        .tasks
        .iter()
        .filter(|task| task.status == TaskStatus::Open)
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
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" CARRY FORWARD "),
        )
        .highlight_symbol(">> ");

    f.render_widget(list, area);
}

fn render_summary(f: &mut Frame, app: &AppState, area: Rect) {
    let completed_blocks = app
        .day
        .blocks
        .iter()
        .filter(|block| block.status == BlockStatus::Done)
        .count();

    let dropped = app
        .day
        .tasks
        .iter()
        .filter(|task| task.status == TaskStatus::Dropped)
        .count();

    let lines = vec![
        Line::from(Span::styled(
            "Review",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.day.date.to_string()),
        Line::from(""),
        Line::from(format!(
            "Completed tasks: {}",
            app.day.completed_tasks().len()
        )),
        Line::from(format!("Priority open: {}", app.day.priority_tasks().len())),
        Line::from(format!("Completed blocks: {}", completed_blocks)),
        Line::from(format!("Dropped tasks: {}", dropped)),
        Line::from(format!("Focus time: {}m", app.day.focus_minutes)),
        Line::from(""),
        Line::from(Span::styled(
            "Notes",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let mut all_lines = lines;

    for note in &app.day.notes {
        all_lines.push(Line::from(format!("- {}", note.text)));
    }

    let panel = Paragraph::new(all_lines)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" SUMMARY "));

    f.render_widget(panel, area);
}

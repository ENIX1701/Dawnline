use crate::models::{BlockStatus, TaskStatus};
use crate::state::AppState;
use crate::theme::DawnTheme;
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let theme = DawnTheme::dawn();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    render_now(f, app, chunks[0], theme);
    render_queue(f, app, chunks[1], theme);
}

fn render_now(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let active = app.day.active_block();
    let next = app
        .day
        .blocks
        .iter()
        .find(|block| block.status == BlockStatus::Planned);

    let mut lines = vec![
        Line::from(Span::styled("now", theme.accent())),
        Line::from(""),
        Line::from(Span::styled(
            active
                .map(|block| block.title.clone())
                .unwrap_or_else(|| "No active block".to_string()),
            theme.text(),
        )),
    ];

    if let Some(block) = active
        && let Some(intent) = &block.intent
    {
        lines.push(Line::from(Span::styled(intent.clone(), theme.muted())));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("focus {}", format_minutes(app.day.focus_minutes)),
        theme.muted(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("next", theme.muted())));

    if let Some(block) = next {
        lines.push(Line::from(vec![
            Span::styled(format!("{:<8}", block.timing), theme.faint()),
            Span::styled(block.title.clone(), theme.muted()),
        ]));
    } else {
        lines.push(Line::from(Span::styled("Nothing queued", theme.faint())));
    }

    f.render_widget(Paragraph::new(lines), area);
}

fn render_queue(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let tasks = app.day.active_tasks();

    if tasks.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(Span::styled("queue", theme.accent())),
            Line::from(""),
            Line::from(Span::styled("no active tasks", theme.muted())),
            Line::from(Span::styled(
                ": add task     define the next move",
                theme.faint(),
            )),
        ]);

        f.render_widget(empty, area);
        return;
    }

    let mut items = Vec::new();

    items.push(ListItem::new(Line::from(Span::styled(
        "queue",
        theme.accent(),
    ))));
    items.push(ListItem::new(Line::from("")));

    for task in tasks {
        let style = if task.priority {
            theme.priority()
        } else {
            theme.text()
        };

        // let prefix = if task.priority { "  " } else { "  " };
        let prefix = "  ";

        let status = match task.status {
            TaskStatus::Open => "",
            TaskStatus::Done => "done",
            TaskStatus::Dropped => "dropped",
            TaskStatus::Removed => "removed",
        };

        items.push(ListItem::new(Line::from(vec![
            Span::styled(prefix, theme.faint()),
            Span::styled(task.title.clone(), style),
            Span::styled(format!(" {}", status), theme.faint()),
        ])));
    }

    let list = List::new(items)
        .highlight_symbol("| ")
        .highlight_style(theme.selected());

    let mut state = app.task_state;
    f.render_stateful_widget(list, area, &mut state);
}

fn format_minutes(minutes: u32) -> String {
    if minutes < 60 {
        return format!("{}m", minutes);
    }

    let hours = minutes / 60;
    let rem = minutes % 60;

    if rem == 0 {
        format!("{}h", hours)
    } else {
        format!("{}h {}m", hours, rem)
    }
}

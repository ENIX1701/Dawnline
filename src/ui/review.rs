use crate::models::{BlockStatus, TaskStatus};
use crate::state::AppState;
use crate::theme::DawnTheme;
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let theme = DawnTheme::dawn();

    if area.width < 94 {
        render_narrow(f, app, area, theme);
        return;
    }

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(35),
            Constraint::Percentage(25),
        ])
        .split(area);

    render_completed(f, app, columns[0], theme);
    render_carry_forward(f, app, columns[1], theme);
    render_session(f, app, columns[2], theme);
}

fn render_narrow(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(area);

    render_completed(f, app, rows[0], theme);
    render_carry_forward(f, app, rows[1], theme);
    render_session(f, app, rows[2], theme);
}

fn render_completed(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let mut items = vec![
        ListItem::new(Line::from(Span::styled("completed", theme.accent()))),
        ListItem::new(Line::from(Span::styled(
            "-----------------------",
            theme.faint(),
        ))),
    ];

    let completed: Vec<_> = app
        .day
        .tasks
        .iter()
        .filter(|task| task.status == TaskStatus::Done)
        .collect();

    if completed.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "no completed tasks yet",
            theme.muted(),
        ))));
    } else {
        for task in completed {
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  ", theme.faint()),
                Span::styled(task.title.clone(), theme.text()),
            ])));
        }
    }

    f.render_widget(List::new(items), area);
}

fn render_carry_forward(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let mut items = vec![
        ListItem::new(Line::from(Span::styled("carry forward", theme.accent()))),
        ListItem::new(Line::from(Span::styled(
            "-------------------------",
            theme.faint(),
        ))),
    ];

    let open: Vec<_> = app
        .day
        .tasks
        .iter()
        .filter(|task| task.status == TaskStatus::Open)
        .collect();

    if open.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "nothing waiting",
            theme.muted(),
        ))));
    } else {
        for task in open {
            let style = if task.priority {
                theme.priority()
            } else {
                theme.text()
            };

            items.push(ListItem::new(Line::from(vec![
                Span::styled("  ", theme.faint()),
                Span::styled(task.title.clone(), style),
            ])));
        }
    }

    f.render_widget(List::new(items), area);
}

fn render_session(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
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

    let mut lines = vec![
        Line::from(Span::styled("session", theme.accent())),
        Line::from(Span::styled("---------------", theme.faint())),
        metric_line(theme, "focus time", format_minutes(app.day.focus_minutes)),
        metric_line(theme, "blocks closed", completed_blocks.to_string()),
        metric_line(theme, "tasks dropped", dropped.to_string()),
        Line::from(""),
        Line::from(Span::styled("notes", theme.accent())),
        Line::from(Span::styled("---------------", theme.faint())),
    ];

    if app.day.notes.is_empty() {
        lines.push(Line::from(Span::styled("no notes", theme.muted())));
    } else {
        for note in &app.day.notes {
            lines.push(Line::from(vec![
                Span::styled("  ", theme.faint()),
                Span::styled(note.text.clone(), theme.muted()),
            ]));
        }
    }

    f.render_widget(Paragraph::new(lines), area);
}

fn metric_line(theme: DawnTheme, label: &'static str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{:<15}", label), theme.muted()),
        Span::styled(value, theme.text()),
    ])
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

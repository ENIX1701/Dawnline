use crate::models::BlockStatus;
use crate::state::{ActivePane, AppState};
use crate::theme::DawnTheme;
use ratatui::{
    prelude::*,
    widgets::{Block as WidgetBlock, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let area = area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(2),
            Constraint::Percentage(55),
        ])
        .split(area);

    render_now(f, app, chunks[0], theme);
    render_queue(f, app, chunks[2], theme);
}

fn render_now(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let selected = app.active_pane == ActivePane::Timeline;
    let area = pane_area(f, area, theme, selected);
    let title_style = if selected {
        theme.accent()
    } else {
        theme.muted()
    };
    let rail = if selected { "| " } else { "  " };
    let active = app.day.active_block();
    let next = app
        .day
        .blocks
        .iter()
        .find(|block| block.status == BlockStatus::Planned);

    let mut lines = vec![
        Line::from(vec![
            Span::styled(rail, theme.accent()),
            Span::styled("now", title_style),
        ]),
        Line::from(Span::styled("  ---", theme.faint())),
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
    let active = app.active_pane == ActivePane::Tasks;
    let area = pane_area(f, area, theme, active);
    let title_style = if active {
        theme.accent()
    } else {
        theme.muted()
    };
    let rail = if active { "| " } else { "  " };

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled(rail, theme.accent()),
                Span::styled("queue", title_style),
            ]),
            Line::from(Span::styled("  ------------", theme.faint())),
            Line::from(""),
        ]),
        rows[0],
    );

    let tasks = app.day.active_tasks();

    if tasks.is_empty() {
        f.render_widget(
            Paragraph::new(vec![
                Line::from(Span::styled("no active tasks", theme.muted())),
                Line::from(Span::styled(
                    ": add task     define the next move",
                    theme.faint(),
                )),
            ]),
            rows[1],
        );
        return;
    }

    let items: Vec<ListItem> = tasks
        .into_iter()
        .map(|task| {
            let style = if task.priority {
                theme.priority()
            } else {
                theme.text()
            };

            ListItem::new(Line::from(vec![
                Span::styled("  ", theme.faint()),
                Span::styled(task.title.clone(), style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .highlight_symbol(if active { "| " } else { "  " })
        .highlight_style(theme.selected());

    let mut state = app.task_state;
    f.render_stateful_widget(list, rows[1], &mut state);
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

fn pane_area(f: &mut Frame, area: Rect, theme: DawnTheme, active: bool) -> Rect {
    if area.width < 6 || area.height < 3 {
        return area;
    }

    let border_style = if active { theme.muted() } else { theme.faint() };

    f.render_widget(
        WidgetBlock::default()
            .borders(Borders::ALL)
            .border_style(border_style),
        area,
    );

    area.inner(Margin {
        horizontal: 2,
        vertical: 1,
    })
}

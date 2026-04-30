use crate::models::{Block, BlockStatus, Task};
use crate::state::AppState;
use crate::theme::DawnTheme;
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let theme = DawnTheme::dawn();

    if area.width < 92 {
        render_narrow(f, app, area, theme);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(7)])
        .split(area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(rows[0]);

    render_timeline(f, app, columns[0], theme);
    render_tasks(f, app, columns[1], theme);
    render_details(f, app, rows[1], theme);
}

fn render_narrow(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(35),
            Constraint::Min(5),
        ])
        .split(area);

    render_timeline(f, app, rows[0], theme);
    render_tasks(f, app, rows[1], theme);
    render_details(f, app, rows[2], theme);
}

fn render_timeline(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let mut items = vec![
        ListItem::new(Line::from(Span::styled("timeline", theme.accent()))),
        ListItem::new(Line::from(Span::styled("-------------", theme.faint()))),
    ];

    if app.day.blocks.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "no line set for today",
            theme.muted(),
        ))));
        items.push(ListItem::new(Line::from(Span::styled(
            ": add block now     define the first move",
            theme.faint(),
        ))));
    }

    for block in &app.day.blocks {
        let title_style = match block.status {
            BlockStatus::Planned => theme.text(),
            BlockStatus::Active => theme.accent(),
            BlockStatus::Done => theme.muted(),
        };

        items.push(ListItem::new(Line::from(vec![
            Span::styled(format!("{:<8}", block.timing), theme.muted()),
            Span::styled(block.title.clone(), title_style),
        ])));

        if block.status == BlockStatus::Active
            && let Some(intent) = &block.intent
        {
            items.push(ListItem::new(Line::from(vec![
                Span::styled("        ", theme.faint()),
                Span::styled(format!("intent: {}", intent), theme.muted()),
            ])));
        }
    }

    let list = List::new(items)
        .highlight_symbol("| ")
        .highlight_style(theme.selected());

    let mut state = app.block_state;
    f.render_stateful_widget(list, area, &mut state);
}

fn render_tasks(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let tasks = app.day.active_tasks();
    let priority: Vec<&Task> = tasks.iter().copied().filter(|task| task.priority).collect();
    let normal: Vec<&Task> = tasks
        .iter()
        .copied()
        .filter(|task| !task.priority)
        .collect();

    let mut items = vec![
        ListItem::new(Line::from(Span::styled("tasks", theme.accent()))),
        ListItem::new(Line::from(Span::styled("------------", theme.faint()))),
        ListItem::new(Line::from(Span::styled("priority", theme.muted()))),
    ];

    if priority.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "none",
            theme.faint(),
        ))));
    } else {
        for task in priority {
            items.push(task_line(task, theme.priority()));
        }
    }

    items.push(ListItem::new(Line::from("")));
    items.push(ListItem::new(Line::from(Span::styled(
        "queue",
        theme.muted(),
    ))));

    if normal.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "empty",
            theme.faint(),
        ))));
    } else {
        for task in normal {
            items.push(task_line(task, theme.text()));
        }
    }

    let list = List::new(items)
        .highlight_symbol("| ")
        .highlight_style(theme.selected());

    let mut state = app.task_state;
    f.render_stateful_widget(list, area, &mut state);
}

fn task_line(task: &Task, style: Style) -> ListItem<'static> {
    ListItem::new(Line::from(vec![
        Span::raw("  "),
        Span::styled(task.title.clone(), style),
    ]))
}

fn render_details(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let selected = selected_block(app);

    let lines = if let Some(block) = selected {
        let mut lines = vec![
            Line::from(Span::styled("details", theme.accent())),
            Line::from(Span::styled("-----------", theme.faint())),
            Line::from(Span::styled(block.title.clone(), theme.text())),
        ];

        if let Some(intent) = &block.intent {
            lines.push(Line::from(Span::styled(intent.clone(), theme.muted())));
        } else {
            lines.push(Line::from(Span::styled("no intent set", theme.faint())));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("blocks ", theme.faint()),
            Span::styled(app.day.blocks.len().to_string(), theme.muted()),
            Span::styled("   open ", theme.faint()),
            Span::styled(app.day.active_tasks().len().to_string(), theme.muted()),
            Span::styled("   notes ", theme.faint()),
            Span::styled(app.day.notes.len().to_string(), theme.muted()),
        ]));

        lines
    } else {
        vec![
            Line::from(Span::styled("details", theme.accent())),
            Line::from(Span::styled("-------------", theme.faint())),
            Line::from(Span::styled(
                "select or add a block to shape the day",
                theme.muted(),
            )),
        ]
    };

    f.render_widget(Paragraph::new(lines), area);
}

fn selected_block(app: &AppState) -> Option<&Block> {
    app.block_state
        .selected()
        .and_then(|index| app.day.blocks.get(index))
        .or_else(|| app.day.blocks.first())
}

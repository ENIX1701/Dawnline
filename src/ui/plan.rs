use crate::models::{Block, BlockStatus, Task};
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

    if area.width < 92 {
        render_narrow(f, app, area, theme);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(7),
        ])
        .split(area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55),
            Constraint::Length(2),
            Constraint::Percentage(45),
        ])
        .split(rows[0]);

    render_timeline(f, app, columns[0], theme);
    render_tasks(f, app, columns[2], theme);
    render_details(f, app, rows[2], theme);
}

fn render_narrow(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(1),
            Constraint::Percentage(32),
            Constraint::Length(1),
            Constraint::Min(6),
        ])
        .split(area);

    render_timeline(f, app, rows[0], theme);
    render_tasks(f, app, rows[2], theme);
    render_details(f, app, rows[4], theme);
}

fn render_timeline(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let active = app.active_pane == ActivePane::Timeline;
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
                Span::styled("timeline", title_style),
            ]),
            Line::from(Span::styled("  --------", theme.faint())),
            Line::from(""),
        ]),
        rows[0],
    );

    if app.day.blocks.is_empty() {
        f.render_widget(
            Paragraph::new(vec![
                Line::from(Span::styled("no line set for today", theme.muted())),
                Line::from(Span::styled(
                    ": add block now     define the first move",
                    theme.faint(),
                )),
            ]),
            rows[1],
        );
        return;
    }

    let items: Vec<ListItem> = app
        .day
        .blocks
        .iter()
        .map(|block| {
            let title_style = match block.status {
                BlockStatus::Planned => theme.text(),
                BlockStatus::Active => theme.accent(),
                BlockStatus::Done => theme.muted(),
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<8}", block.timing), theme.muted()),
                Span::styled(block.title.clone(), title_style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .highlight_symbol(if active { "| " } else { "  " })
        .highlight_style(theme.selected());

    let mut state = app.block_state;
    f.render_stateful_widget(list, rows[1], &mut state);
}

fn render_tasks(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let active = app.active_pane == ActivePane::Tasks;
    let area = pane_area(f, area, theme, active);
    let title_style = if active {
        theme.accent()
    } else {
        theme.muted()
    };
    let rail = if active { "| " } else { "  " };

    let tasks = app.day.active_tasks();
    let priority_count = tasks.iter().filter(|task| task.priority).count();
    let normal_count = tasks.len().saturating_sub(priority_count);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(1)])
        .split(area);

    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled(rail, theme.accent()),
                Span::styled("tasks", title_style),
            ]),
            Line::from(Span::styled("  ------------", theme.faint())),
            Line::from(vec![
                Span::styled("  priority ", theme.muted()),
                Span::styled(priority_count.to_string(), theme.faint()),
                Span::styled("   queue ", theme.muted()),
                Span::styled(normal_count.to_string(), theme.faint()),
            ]),
            Line::from(""),
        ]),
        rows[0],
    );

    if tasks.is_empty() {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled("empty", theme.faint()))),
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

            task_line(task, style)
        })
        .collect();

    let list = List::new(items)
        .highlight_symbol(if active { "| " } else { "  " })
        .highlight_style(theme.selected());

    let mut state = app.task_state;
    f.render_stateful_widget(list, rows[1], &mut state);
}

fn task_line(task: &Task, style: Style) -> ListItem<'static> {
    ListItem::new(Line::from(vec![
        Span::raw("  "),
        Span::styled(task.title.clone(), style),
    ]))
}

fn render_details(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let area = pane_area(f, area, theme, false);
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

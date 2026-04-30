use crate::state::AppState;
use crate::theme::DawnTheme;
use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

pub fn render_command(f: &mut Frame, app: &AppState) {
    let theme = DawnTheme::dawn();
    let root = f.area();
    let height = 3;
    let area = Rect {
        x: root.x,
        y: root.y + root.height.saturating_sub(height),
        width: root.width,
        height,
    };

    f.render_widget(Clear, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "─".repeat(rows[0].width as usize),
            theme.faint(),
        ))),
        rows[0],
    );

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(":", theme.accent()),
            Span::styled(format!("{}█", app.command_buffer), theme.text()),
        ])),
        rows[1],
    );
}

pub fn render_help(f: &mut Frame) {
    let theme = DawnTheme::dawn();
    let root = f.area();
    let height = 9;
    let area = Rect {
        x: root.x,
        y: root.y + root.height.saturating_sub(height),
        width: root.width,
        height,
    };

    f.render_widget(Clear, area);

    let lines = vec![
        Line::from(Span::styled("Help", theme.accent())),
        Line::from(Span::styled("─".repeat(area.width as usize), theme.faint())),
        Line::from("enter     begin execution from plan"),
        Line::from("tab       switch pane"),
        Line::from("space     complete selected task"),
        Line::from("s         start selected block"),
        Line::from("d         drop selected task"),
        Line::from("f         finish execution / finish day"),
        Line::from(":         command     ? help     q quit"),
    ];

    f.render_widget(Paragraph::new(lines), area);
}

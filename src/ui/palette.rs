use crate::state::AppState;
use crate::theme::DawnTheme;
use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

pub fn render_command(f: &mut Frame, app: &AppState, theme: DawnTheme) {
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
            "-".repeat(rows[0].width as usize),
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

pub fn render_help(f: &mut Frame, theme: DawnTheme) {
    let root = f.area();
    let height = 13;
    let area = Rect {
        x: root.x,
        y: root.y + root.height.saturating_sub(height),
        width: root.width,
        height,
    };

    f.render_widget(Clear, area);

    let inner = area.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let lines = vec![
        Line::from(Span::styled("help", theme.accent())),
        Line::from(Span::styled(
            "-".repeat(inner.width as usize),
            theme.faint(),
        )),
        help_line(theme, "enter", "begin execution from plan"),
        help_line(theme, "tab", "switch pane"),
        help_line(theme, "space", "complete selected task"),
        help_line(theme, "s", "start selected block"),
        help_line(theme, "d", "drop selected task"),
        help_line(theme, "t", "start focus timer in execute"),
        help_line(theme, "f", "finish execution / finish day"),
        Line::from(""),
        Line::from(vec![
            Span::styled(":", theme.accent()),
            Span::styled(" command    ", theme.muted()),
            Span::styled("?", theme.accent()),
            Span::styled(" help    ", theme.muted()),
            Span::styled("q", theme.accent()),
            Span::styled(" quit", theme.muted()),
        ]),
    ];

    f.render_widget(Paragraph::new(lines), inner);
}

fn help_line(theme: DawnTheme, key: &'static str, label: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{:<8}", key), theme.accent()),
        Span::styled(label, theme.muted()),
    ])
}

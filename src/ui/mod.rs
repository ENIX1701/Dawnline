use crate::state::{AppState, CurrentScreen};
use crate::theme::DawnTheme;
use chrono::Local;
use ratatui::{prelude::*, widgets::Paragraph};

pub mod execute;
pub mod palette;
pub mod plan;
pub mod review;

pub fn draw(f: &mut Frame, app: &AppState, theme: DawnTheme, tagline: &str) {
    if app.day.day_finished {
        render_finished_day(f, theme, tagline);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    render_header(f, app, chunks[0], theme, tagline);

    match app.current_screen {
        CurrentScreen::Plan => plan::render(f, app, chunks[1], theme),
        CurrentScreen::Execute => execute::render(f, app, chunks[1], theme),
        CurrentScreen::Review => review::render(f, app, chunks[1], theme),
    }

    render_footer(f, app, chunks[2], theme);

    if app.command_mode {
        palette::render_command(f, app, theme);
    }

    if app.show_help {
        palette::render_help(f, theme);
    }
}

fn render_header(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme, tagline: &str) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(14),
            Constraint::Min(1),
            Constraint::Length(12),
        ])
        .split(rows[0]);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled("dawnline", theme.accent()))),
        columns[0],
    );

    let tagline = if columns[1].width as usize >= tagline.len() {
        tagline.to_string()
    } else {
        String::new()
    };

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(tagline, theme.muted())))
            .alignment(Alignment::Center),
        columns[1],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            app.day.date.format("%a %d %b").to_string(),
            theme.muted(),
        )))
        .alignment(Alignment::Right),
        columns[2],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "-".repeat(rows[1].width as usize),
            theme.faint(),
        )))
        .alignment(Alignment::Center),
        rows[1],
    );

    f.render_widget(Paragraph::new(flow_line(app, theme)), rows[2]);
}

fn flow_line(app: &AppState, theme: DawnTheme) -> Line<'static> {
    let plan = if app.current_screen == CurrentScreen::Plan {
        Span::styled("Plan", theme.accent())
    } else {
        Span::styled("Plan", theme.muted())
    };

    let execute = if app.current_screen == CurrentScreen::Execute {
        Span::styled("Execute", theme.accent())
    } else {
        Span::styled("Execute", theme.muted())
    };

    let review = if app.current_screen == CurrentScreen::Review {
        Span::styled("Review", theme.accent())
    } else {
        Span::styled("Review", theme.muted())
    };

    Line::from(vec![
        plan,
        Span::styled(" -> ", theme.faint()),
        execute,
        Span::styled(" -> ", theme.faint()),
        review,
    ])
}

fn render_footer(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
    let mut spans = Vec::new();

    match app.current_screen {
        CurrentScreen::Plan => {
            push_hint(&mut spans, theme, "tab", "pane");
            push_hint(&mut spans, theme, "a", "add");
            push_hint(&mut spans, theme, "s", "start");
            push_hint(&mut spans, theme, "enter", "execute");
        }
        CurrentScreen::Execute => {
            push_hint(&mut spans, theme, "space", "done");
            push_hint(&mut spans, theme, "t", "focus");
            push_hint(&mut spans, theme, "f", "finish");
            push_hint(&mut spans, theme, ":", "command");
        }
        CurrentScreen::Review => {
            push_hint(&mut spans, theme, "tab", "pane");
            push_hint(&mut spans, theme, "d", "drop");
            push_hint(&mut spans, theme, "x", "remove");
            push_hint(&mut spans, theme, "n", "new session");
            push_hint(&mut spans, theme, "f", "finish day");
            push_hint(&mut spans, theme, ":", "command");
        }
    }

    push_hint(&mut spans, theme, "?", "help");
    push_hint(&mut spans, theme, "q", "quit");

    spans.push(Span::styled(" | ", theme.faint()));
    spans.push(Span::styled(app.status_message.clone(), theme.muted()));

    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn push_hint(
    spans: &mut Vec<Span<'static>>,
    theme: DawnTheme,
    key: &'static str,
    label: &'static str,
) {
    spans.push(Span::styled(key, theme.accent()));
    spans.push(Span::styled(format!(" {}  ", label), theme.muted()));
}

fn render_finished_day(f: &mut Frame, theme: DawnTheme, tagline: &str) {
    let area = f.area();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(3),
            Constraint::Percentage(55),
        ])
        .split(area);

    let now = Local::now().format("%H:%M").to_string();
    let lines = vec![
        Line::from(Span::styled(now, theme.muted())).centered(),
        Line::from(Span::styled(tagline.to_string(), theme.accent())).centered(),
        Line::from(Span::styled("q quit", theme.faint())).centered(),
    ];

    f.render_widget(Paragraph::new(lines), rows[1]);
}

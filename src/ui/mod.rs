use crate::state::{AppState, CurrentScreen};
use crate::theme::DawnTheme;
use ratatui::{prelude::*, widgets::Paragraph};

pub mod execute;
pub mod palette;
pub mod plan;
pub mod review;

pub fn draw(f: &mut Frame, app: &AppState) {
    let theme = DawnTheme::dawn();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    render_header(f, app, chunks[0], theme);

    match app.current_screen {
        CurrentScreen::Plan => plan::render(f, app, chunks[1]),
        CurrentScreen::Execute => execute::render(f, app, chunks[1]),
        CurrentScreen::Review => review::render(f, app, chunks[1]),
    }

    render_footer(f, app, chunks[2], theme);

    if app.command_mode {
        palette::render_command(f, app);
    }

    if app.show_help {
        palette::render_help(f);
    }
}

fn render_header(f: &mut Frame, app: &AppState, area: Rect, theme: DawnTheme) {
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

    f.render_widget(
        Paragraph::new(Line::from(Span::styled("know what matters", theme.muted())))
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
            theme.muted(),
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
            push_hint(&mut spans, theme, "f", "finish");
            push_hint(&mut spans, theme, ":", "command");
        }
        CurrentScreen::Review => {
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

use crate::state::AppState;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_command(f: &mut Frame, app: &AppState) {
    let area = centered_rect(70, 20, f.area());
    f.render_widget(Clear, area);

    let input = Paragraph::new(Line::from(vec![
        Span::styled(": ", Style::default().fg(Color::Cyan)),
        Span::styled(
            format!("{}█", app.command_buffer),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(" COMMAND "),
    );

    f.render_widget(input, area);
}

pub fn render_help(f: &mut Frame) {
    let area = centered_rect(60, 55, f.area());
    f.render_widget(Clear, area);

    let lines = vec![
        Line::from("tab       switch mode"),
        Line::from(":         command palette"),
        Line::from("space     complete selected task"),
        Line::from("s         start selected block"),
        Line::from("d         drop selected task"),
        Line::from("x         remove selected task"),
        Line::from("p/e/r     plan / execute / review"),
        Line::from("q         quit"),
        Line::from(""),
        Line::from("add task Ship auth fix"),
        Line::from("add priority task Patch billing edge case"),
        Line::from("add block 13:00 Review metrics"),
        Line::from("note Refresh path looks suspicious"),
        Line::from("focus 45"),
        Line::from("finish"),
    ];

    let help = Paragraph::new(lines)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" HELP "));

    f.render_widget(help, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

use ratatui::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct DawnTheme {
    pub fg: Color,
    pub muted: Color,
    pub faint: Color,
    pub accent: Color,
    pub soft: Color,
}

impl DawnTheme {
    pub fn dawn() -> Self {
        Self {
            fg: Color::Rgb(218, 216, 207),
            muted: Color::Rgb(142, 139, 132),
            faint: Color::Rgb(72, 70, 132),
            accent: Color::Rgb(213, 183, 122),
            soft: Color::Rgb(174, 184, 178),
        }
    }

    pub fn text(self) -> Style {
        Style::default().fg(self.fg)
    }

    pub fn muted(self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn faint(self) -> Style {
        Style::default().fg(self.faint)
    }

    pub fn accent(self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected(self) -> Style {
        Style::default().fg(self.fg).add_modifier(Modifier::BOLD)
    }

    pub fn priority(self) -> Style {
        Style::default().fg(self.soft).add_modifier(Modifier::BOLD)
    }
}

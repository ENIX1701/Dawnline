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
            faint: Color::Rgb(72, 70, 67),
            accent: Color::Rgb(213, 183, 122),
            soft: Color::Rgb(174, 184, 178),
        }
    }

    pub fn opal() -> Self {
        Self {
            fg: Color::Rgb(219, 224, 218),
            muted: Color::Rgb(144, 151, 146),
            faint: Color::Rgb(66, 72, 68),
            accent: Color::Rgb(176, 207, 196),
            soft: Color::Rgb(219, 188, 202),
        }
    }

    pub fn mist() -> Self {
        Self {
            fg: Color::Rgb(215, 219, 223),
            muted: Color::Rgb(139, 146, 153),
            faint: Color::Rgb(65, 69, 74),
            accent: Color::Rgb(170, 190, 215),
            soft: Color::Rgb(201, 185, 214),
        }
    }

    pub fn named(name: &str) -> Self {
        match name.trim().to_ascii_lowercase().as_str() {
            "opal" => Self::opal(),
            "mist" => Self::mist(),
            _ => Self::dawn(),
        }
    }

    pub fn with_accent_name(mut self, accent: &str) -> Self {
        self.accent = match accent.trim().to_ascii_lowercase().as_str() {
            "gold" => Color::Rgb(213, 183, 122),
            "rose" => Color::Rgb(219, 164, 176),
            "sage" | "opal" => Color::Rgb(176, 207, 196),
            "blue" | "mist" => Color::Rgb(170, 190, 215),
            "lilac" => Color::Rgb(201, 185, 214),
            _ => self.accent,
        };

        self
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

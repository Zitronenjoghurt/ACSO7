use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub background: Rgb,
    pub normal: Rgb,
    pub good: Rgb,
    pub danger: Rgb,
    pub error: Rgb,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Rgb::new(0x0A, 0x0E, 0x14), // near-black
            normal: Rgb::new(0x05, 0xD9, 0xE8),     // neon cyan
            good: Rgb::new(0x39, 0xFF, 0x14),       // neon green
            danger: Rgb::new(0xFF, 0xB0, 0x00),     // neon amber
            error: Rgb::new(0xFF, 0x2A, 0x6D),      // neon magenta/red
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub trait ThemeStyles {
    fn background(&self) -> Style;
    fn normal(&self) -> Style;
    fn good(&self) -> Style;
    fn danger(&self) -> Style;
    fn error(&self) -> Style;
}

impl ThemeStyles for Theme {
    fn background(&self) -> Style {
        Style::new().bg(color(self.background))
    }

    fn normal(&self) -> Style {
        self.background().fg(color(self.normal))
    }

    fn good(&self) -> Style {
        self.background().fg(color(self.good))
    }

    fn danger(&self) -> Style {
        self.background().fg(color(self.danger))
    }

    fn error(&self) -> Style {
        self.background().fg(color(self.error))
    }
}

fn color(rgb: Rgb) -> Color {
    Color::Rgb(rgb.r, rgb.g, rgb.b)
}

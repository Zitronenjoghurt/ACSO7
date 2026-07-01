use crate::ui::theme::{Theme, ThemeStyles};
use crate::world::ship::alert::{Alert, AlertLevel};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Text, Widget};
use ratatui::style::{Modifier, Style};

pub struct Alerts<'a> {
    alerts: &'a [Alert],
    theme: &'a Theme,
    indent: usize,
}

impl<'a> Alerts<'a> {
    pub fn new(alerts: &'a [Alert], theme: &'a Theme) -> Self {
        Self {
            alerts,
            theme,
            indent: 0,
        }
    }

    pub fn indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    pub fn top_level(&self) -> Option<AlertLevel> {
        self.alerts.iter().map(|a| a.level).max()
    }

    pub fn lines(&self) -> Vec<Line<'static>> {
        self.alerts
            .iter()
            .map(|a| alert_line(a, self.theme, self.indent))
            .collect()
    }
}

impl Widget for Alerts<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Text::from(self.lines()).render(area, buf);
    }
}

pub fn alert_style(level: AlertLevel, theme: &Theme) -> Style {
    match level {
        AlertLevel::Critical => theme
            .error()
            .add_modifier(Modifier::REVERSED | Modifier::BOLD),
        AlertLevel::Warning => theme.danger().add_modifier(Modifier::BOLD),
    }
}

fn alert_line(alert: &Alert, theme: &Theme, indent: usize) -> Line<'static> {
    let symbol = match alert.level {
        AlertLevel::Critical => "‼",
        AlertLevel::Warning => "⚠",
    };
    let text = format!("{}{} {} ", " ".repeat(indent), symbol, alert.label);
    Line::from(text).style(alert_style(alert.level, theme))
}

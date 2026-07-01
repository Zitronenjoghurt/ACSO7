use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Text, Widget};
use ratatui::style::Modifier;
use ratatui::widgets::Padding;

pub struct StatusBar<'a> {
    theme: &'a Theme,
    mission_secs: f64,
}

impl<'a> StatusBar<'a> {
    pub fn new(theme: &'a Theme, mission_secs: f64) -> Self {
        Self {
            theme,
            mission_secs,
        }
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = Panel::new("STATUS", self.theme)
            .focused(false)
            .title_left()
            .padding(Padding::new(1, 1, 0, 0))
            .render(area, buf);

        let lines = vec![
            self.field("MISSION TIME", mission_time(self.mission_secs), false),
            self.field("DESTINATION", "---".to_string(), true),
            self.field("TRAVEL", "[░░░░░░░░░░]   0%".to_string(), true),
        ];
        Text::from(lines).render(inner, buf);
    }
}

impl<'a> StatusBar<'a> {
    fn field(&self, label: &'a str, value: String, pending: bool) -> Line<'a> {
        let style = if pending {
            self.theme.normal().add_modifier(Modifier::DIM)
        } else {
            self.theme.good()
        };
        Line::from(vec![
            Span::styled(format!("{label:<14}"), self.theme.normal()),
            Span::styled(value, style),
        ])
    }
}

fn mission_time(secs: f64) -> String {
    let total = secs.max(0.0) as u64;
    let days = total / 86400;
    let hours = (total % 86400) / 3600;
    let mins = (total % 3600) / 60;
    let s = total % 60;
    if days > 0 {
        format!("T+ {days}d {hours:02}:{mins:02}:{s:02}")
    } else {
        format!("T+ {hours:02}:{mins:02}:{s:02}")
    }
}

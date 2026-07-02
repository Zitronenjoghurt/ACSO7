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
    paused: bool,
}

impl<'a> StatusBar<'a> {
    pub fn new(theme: &'a Theme, mission_secs: f64) -> Self {
        Self {
            theme,
            mission_secs,
            paused: false,
        }
    }

    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
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
            self.mission_time_field(),
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

    fn mission_time_field(&self) -> Line<'a> {
        let value = mission_time(self.mission_secs);
        let style = if self.paused {
            let mut style = self.theme.danger().add_modifier(Modifier::BOLD);
            if !blink_on() {
                style = style.add_modifier(Modifier::DIM);
            }
            style
        } else {
            self.theme.good()
        };
        Line::from(vec![
            Span::styled(format!("{:<14}", "MISSION TIME"), self.theme.normal()),
            Span::styled(value, style),
        ])
    }
}

fn blink_on() -> bool {
    (jiff::Timestamp::now().as_millisecond() / 450).rem_euclid(2) == 0
}

fn mission_time(secs: f64) -> String {
    const DAY: u64 = 86400;
    const MONTH: u64 = 30 * DAY;
    const YEAR: u64 = 365 * DAY;

    let total = secs.max(0.0) as u64;
    let years = total / YEAR;
    let months = (total % YEAR) / MONTH;
    let days = (total % MONTH) / DAY;
    let hours = (total % DAY) / 3600;
    let mins = (total % 3600) / 60;
    let s = total % 60;
    format!("T+ {years:04}y:{months:02}m:{days:02}d {hours:02}:{mins:02}:{s:02}")
}

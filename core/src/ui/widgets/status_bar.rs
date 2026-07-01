use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Text, Widget};
use ratatui::style::Modifier;
use ratatui::widgets::Padding;

pub struct StatusBar<'a> {
    theme: &'a Theme,
}

impl<'a> StatusBar<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
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
            self.field("MISSION TIME", "T+ --:--:--"),
            self.field("DESTINATION", "---"),
            self.field("TRAVEL", "[░░░░░░░░░░]   0%"),
        ];
        Text::from(lines).render(inner, buf);
    }
}

impl<'a> StatusBar<'a> {
    fn field(&self, label: &'a str, value: &'a str) -> Line<'a> {
        let pending = self.theme.normal().add_modifier(Modifier::DIM);
        Line::from(vec![
            Span::styled(format!("{label:<14}"), self.theme.normal()),
            Span::styled(value, pending),
        ])
    }
}

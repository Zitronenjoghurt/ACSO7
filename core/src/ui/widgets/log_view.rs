use crate::ui::log::EventLog;
use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Text, Widget};
use ratatui::widgets::Padding;

pub struct LogView<'a> {
    log: &'a EventLog,
    theme: &'a Theme,
}

impl<'a> LogView<'a> {
    pub fn new(log: &'a EventLog, theme: &'a Theme) -> Self {
        Self { log, theme }
    }
}

impl Widget for LogView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = Panel::new("LOG", self.theme)
            .focused(false)
            .title_left()
            .padding(Padding::new(1, 1, 0, 0))
            .render(area, buf);

        let lines: Vec<Line> = self
            .log
            .recent(inner.height as usize)
            .map(|e| Line::from(e.clone()).style(self.theme.normal()))
            .collect();
        Text::from(lines).render(inner, buf);
    }
}

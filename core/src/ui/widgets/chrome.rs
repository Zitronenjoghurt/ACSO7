use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::padded_line::PaddedLine;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Widget;

const TITLE: &str = "[ Autonomous Colony Ship Operator v7 ]";

pub struct Chrome<'a> {
    theme: &'a Theme,
    footer: &'a str,
}

impl<'a> Chrome<'a> {
    pub fn new(theme: &'a Theme, footer: &'a str) -> Self {
        Self { theme, footer }
    }

    pub fn render(self, area: Rect, buf: &mut Buffer) -> Rect {
        buf.set_style(area, self.theme.background());

        let [header, content, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        PaddedLine::new(TITLE)
            .padding_symbol('═')
            .style(self.theme.normal())
            .render(header, buf);
        PaddedLine::new(self.footer)
            .padding_symbol('═')
            .style(self.theme.normal())
            .render(footer, buf);

        content
    }
}

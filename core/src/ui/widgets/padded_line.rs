use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Widget};
use ratatui::style::Style;

pub struct PaddedLine<'a> {
    pub text: &'a str,
    pub padding_symbol: Option<char>,
    pub style: Style,
}

impl<'a> PaddedLine<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            padding_symbol: None,
            style: Style::default(),
        }
    }

    pub fn padding_symbol(mut self, symbol: char) -> Self {
        self.padding_symbol = Some(symbol);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for PaddedLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = match self.padding_symbol {
            Some(sym) => {
                let label = self.text.to_string();
                let pad = (area.width as usize).saturating_sub(label.chars().count());
                let l: String = std::iter::repeat_n(sym, pad / 2).collect();
                let r: String = std::iter::repeat_n(sym, pad - pad / 2).collect();
                Line::from(format!("{l}{label}{r}"))
            }
            None => Line::from(self.text).centered(),
        };
        line.style(self.style).render(area, buf);
    }
}

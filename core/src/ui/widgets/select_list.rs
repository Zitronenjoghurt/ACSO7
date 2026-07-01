use crate::ui::theme::{Theme, ThemeStyles};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Text, Widget};

pub struct SelectList<'a> {
    theme: &'a Theme,
    items: Vec<String>,
    selected: usize,
    empty: Option<&'a str>,
}

impl<'a> SelectList<'a> {
    pub fn new(theme: &'a Theme, items: Vec<String>, selected: usize) -> Self {
        Self {
            theme,
            items,
            selected,
            empty: None,
        }
    }

    pub fn empty(mut self, message: &'a str) -> Self {
        self.empty = Some(message);
        self
    }
}

impl Widget for SelectList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.items.is_empty() {
            if let Some(message) = self.empty {
                Text::from(Line::from(message).style(self.theme.normal()))
                    .centered()
                    .render(area, buf);
            }
            return;
        }

        let lines: Vec<Line> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                if i == self.selected {
                    Line::from(format!("▶ {item} ◀")).style(self.theme.good())
                } else {
                    Line::from(item.clone()).style(self.theme.normal())
                }
            })
            .collect();
        Text::from(lines).centered().render(area, buf);
    }
}

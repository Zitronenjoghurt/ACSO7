use crate::ui::theme::{Theme, ThemeStyles};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::{Line, Widget};
use ratatui::widgets::{Block, BorderType, Padding};

pub struct Panel<'a> {
    title: &'a str,
    theme: &'a Theme,
    focused: bool,
    title_alignment: Alignment,
    padding: Padding,
    fixed: Option<(u16, u16)>,
    left: Option<Line<'a>>,
    right: Option<Line<'a>>,
    bottom: Option<Line<'a>>,
}

impl<'a> Panel<'a> {
    pub fn new(title: &'a str, theme: &'a Theme) -> Self {
        Self {
            title,
            theme,
            focused: true,
            title_alignment: Alignment::Center,
            padding: Padding::symmetric(2, 1),
            fixed: None,
            left: None,
            right: None,
            bottom: None,
        }
    }

    pub fn left(mut self, left: Line<'a>) -> Self {
        self.left = Some(left);
        self
    }

    pub fn bottom(mut self, bottom: Line<'a>) -> Self {
        self.bottom = Some(bottom);
        self
    }

    pub fn right(mut self, right: Line<'a>) -> Self {
        self.right = Some(right);
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn title_left(mut self) -> Self {
        self.title_alignment = Alignment::Left;
        self
    }

    pub fn fixed(mut self, width: u16, content_rows: u16) -> Self {
        self.fixed = Some((width, content_rows));
        self
    }

    pub fn render(self, area: Rect, buf: &mut Buffer) -> Rect {
        let area = match self.fixed {
            Some((width, rows)) => center(area, width.min(area.width.saturating_sub(4)), rows + 4),
            None => area,
        };
        let border = if self.focused {
            BorderType::Thick
        } else {
            BorderType::Rounded
        };
        let mut block = Block::bordered()
            .border_type(border)
            .border_style(self.theme.normal())
            .title(
                Line::from(format!(" {} ", self.title))
                    .alignment(self.title_alignment)
                    .style(self.theme.good()),
            )
            .padding(self.padding);
        if let Some(left) = self.left {
            block = block.title(left.left_aligned());
        }
        if let Some(right) = self.right {
            block = block.title(right.right_aligned());
        }
        if let Some(bottom) = self.bottom {
            block = block.title_bottom(bottom.centered());
        }
        let inner = block.inner(area);
        block.render(area, buf);
        inner
    }
}

fn center(area: Rect, width: u16, rows: u16) -> Rect {
    let [_, v, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(rows),
        Constraint::Fill(1),
    ])
    .areas(area);
    let [_, h, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .areas(v);
    h
}

use crate::ui::popup::PopupState;
use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::{Panel, center};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Text, Widget};
use ratatui::widgets::Clear;

const MIN_WIDTH: u16 = 44;
const MAX_WIDTH: u16 = 76;

pub struct Popup<'a> {
    state: &'a PopupState,
    theme: &'a Theme,
}

impl<'a> Popup<'a> {
    pub fn new(state: &'a PopupState, theme: &'a Theme) -> Self {
        Self { state, theme }
    }
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let total = self.state.lines.len() as u16;
        let content_width = self.state.lines.iter().map(Line::width).max().unwrap_or(0) as u16;
        let width = (content_width + 6)
            .clamp(MIN_WIDTH, MAX_WIDTH)
            .min(area.width.saturating_sub(4));
        let max_rows = area.height.saturating_sub(6).max(1);
        let rows = total.clamp(1, max_rows);
        let scrollable = total > rows;
        let offset = if scrollable {
            self.state.scroll.min((total - rows) as usize)
        } else {
            0
        };

        let mut panel = Panel::new(&self.state.title, self.theme).bottom(
            Line::from(if scrollable {
                " ↑↓ ., SCROLL · ANY OTHER KEY CLOSES "
            } else {
                " ANY KEY CLOSES "
            })
            .style(self.theme.normal()),
        );
        if scrollable {
            panel = panel.right(
                Line::from(format!(
                    " {}–{}/{} ",
                    offset + 1,
                    offset + rows as usize,
                    total
                ))
                .style(self.theme.normal()),
            );
        }

        let outer = center(area, width, rows + 4);
        Clear.render(outer, buf);
        buf.set_style(outer, self.theme.background());
        let inner = panel.render(outer, buf);

        let end = (offset + inner.height as usize).min(self.state.lines.len());
        let visible: Vec<Line> = self.state.lines[offset..end].to_vec();
        Text::from(visible).render(inner, buf);
    }
}

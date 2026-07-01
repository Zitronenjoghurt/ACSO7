use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use crate::world::ship::resources::{ShipResource, ShipResources};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Text, Widget};
use ratatui::widgets::Padding;
use strum::IntoEnumIterator;

pub struct Resources<'a> {
    res: &'a ShipResources,
    theme: &'a Theme,
}

impl<'a> Resources<'a> {
    pub fn new(res: &'a ShipResources, theme: &'a Theme) -> Self {
        Self { res, theme }
    }
}

impl Widget for Resources<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = Panel::new("RESOURCES", self.theme)
            .focused(false)
            .padding(Padding::new(1, 1, 1, 0))
            .render(area, buf);

        let width = inner.width as usize;
        let lines: Vec<Line> = ShipResource::iter()
            .map(|r| {
                let name = r.short_name();
                let value = format!("{:.1}", self.res.get(&r));
                let field = width.saturating_sub(name.len());
                Line::from(vec![
                    Span::styled(name, self.theme.normal()),
                    Span::styled(format!("{value:>field$}"), self.theme.good()),
                ])
            })
            .collect();
        Text::from(lines).render(inner, buf);
    }
}

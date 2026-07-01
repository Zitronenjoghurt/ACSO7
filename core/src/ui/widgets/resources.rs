use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use crate::world::ship::resources::history::ResourceHistory;
use crate::world::ship::resources::{ShipResource, ShipResources};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Text, Widget};
use ratatui::style::Modifier;
use ratatui::widgets::Padding;
use strum::IntoEnumIterator;

pub struct Resources<'a> {
    res: &'a ShipResources,
    history: &'a ResourceHistory,
    theme: &'a Theme,
    focused: bool,
    showing: bool,
    selected: usize,
}

impl<'a> Resources<'a> {
    pub fn new(res: &'a ShipResources, history: &'a ResourceHistory, theme: &'a Theme) -> Self {
        Self {
            res,
            history,
            theme,
            focused: false,
            showing: false,
            selected: 0,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn showing(mut self, showing: bool) -> Self {
        self.showing = showing;
        self
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }
}

impl Widget for Resources<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = Panel::new("RESOURCES", self.theme)
            .focused(self.focused)
            .padding(Padding::new(1, 1, 1, 0))
            .render(area, buf);

        let width = inner.width as usize;
        let lines: Vec<Line> = ShipResource::iter()
            .enumerate()
            .map(|(i, r)| {
                let is_sel = i == self.selected;
                let (marker, name_style) = if is_sel && self.focused {
                    ("▶ ", self.theme.good())
                } else if is_sel && self.showing {
                    ("▶ ", self.theme.normal().add_modifier(Modifier::DIM))
                } else {
                    ("  ", self.theme.normal())
                };
                let name = r.short_name();
                let (trend, trend_style) = trend(self.history.net_rate(r), self.theme);
                let value = format!("{:.1}", self.res.get(&r));
                let field = width.saturating_sub(4 + name.len());
                Line::from(vec![
                    Span::styled(marker, name_style),
                    Span::styled(name, name_style),
                    Span::styled(format!("{value:>field$} "), self.theme.good()),
                    Span::styled(trend, trend_style),
                ])
            })
            .collect();
        Text::from(lines).render(inner, buf);
    }
}

fn trend(rate: f64, theme: &Theme) -> (&'static str, ratatui::style::Style) {
    if rate > 1e-3 {
        ("▲", theme.good())
    } else if rate < -1e-3 {
        ("▼", theme.error())
    } else {
        (" ", theme.normal())
    }
}

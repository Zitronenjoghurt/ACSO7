use crate::ui::theme::Rgb;
use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use crate::world::ship::resources::history::ResourceHistory;
use crate::world::ship::resources::{ShipResource, ShipResources};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Widget};
use ratatui::style::{Color, Modifier};
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
        for (i, r) in ShipResource::iter().enumerate() {
            let y = inner.y + i as u16;
            if y >= inner.y.saturating_add(inner.height) {
                break;
            }

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
            let line = Line::from(vec![
                Span::styled(marker, name_style),
                Span::styled(name, name_style),
                Span::styled(format!("{value:>field$} "), self.theme.good()),
                Span::styled(trend, trend_style),
            ]);

            let row = Rect::new(inner.x, y, inner.width, 1);
            line.render(row, buf);

            if let Some(cap) = self.res.capacity(&r).filter(|c| *c > 0.0) {
                let ratio = (self.res.get(&r) / cap).clamp(0.0, 1.0);
                let fill_cols = (ratio * inner.width as f64).round() as u16;
                let fill_bg = fill_color(ratio, self.theme);
                for x in inner.x..inner.x + fill_cols {
                    buf[(x, y)].set_bg(fill_bg);
                }
            }
        }
    }
}

fn fill_color(ratio: f64, theme: &Theme) -> Color {
    let accent = if ratio >= 0.66 {
        theme.good
    } else if ratio >= 0.33 {
        theme.danger
    } else {
        theme.error
    };
    let blended = blend(theme.background, accent, 0.3);
    Color::Rgb(blended.r, blended.g, blended.b)
}

fn blend(base: Rgb, tint: Rgb, alpha: f64) -> Rgb {
    let mix = |b: u8, t: u8| (b as f64 * (1.0 - alpha) + t as f64 * alpha).round() as u8;
    Rgb::new(
        mix(base.r, tint.r),
        mix(base.g, tint.g),
        mix(base.b, tint.b),
    )
}

fn trend(rate: f64, theme: &Theme) -> (&'static str, ratatui::style::Style) {
    if rate > 1e-12 {
        ("▲", theme.good())
    } else if rate < -1e-12 {
        ("▼", theme.error())
    } else {
        (" ", theme.normal())
    }
}

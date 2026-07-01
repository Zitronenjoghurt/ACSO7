use crate::app::App;
use crate::input::Input;
use crate::ui::screens::Screen;
use crate::ui::theme::ThemeStyles;
use crate::ui::widgets::chart::{TimeChart, fmt_compact};
use crate::ui::widgets::source_breakdown::SourceBreakdown;
use crate::world::ship::resources::ShipResource;
use crate::world::ship::resources::history::ResourceHistory;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Widget;
use strum::IntoEnumIterator;

const SOURCES_WIDTH: u16 = 26;

pub struct ResourceScreen;

fn selected(app: &App) -> ShipResource {
    ShipResource::iter()
        .nth(app.ui.resource_selected)
        .unwrap_or(ShipResource::Power)
}

impl Screen for ResourceScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;
        let ship = &app.world.ship;
        let resource = selected(app);
        let tier = app.ui.history_tier;
        let history = &ship.history;

        let stock = history.stock_series(tier, resource);
        let (produced, consumed) = history.flow_series(tier, resource);
        let span = span_label(history.span_secs(tier));

        let ys: Vec<f64> = stock.iter().map(|p| p.1).collect();
        let now = ys.last().copied().unwrap_or(0.0);
        let min = ys.iter().copied().fold(f64::INFINITY, f64::min);
        let max = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let avg = if ys.is_empty() {
            0.0
        } else {
            ys.iter().sum::<f64>() / ys.len() as f64
        };

        let prod = produced.last().map(|p| p.1).unwrap_or(0.0);
        let cons = consumed.last().map(|p| p.1).unwrap_or(0.0);
        let net = prod - cons;
        let (net_style, sign) = if net >= 0.0 {
            (theme.good(), "+")
        } else {
            (theme.error(), "-")
        };

        let [charts, sources_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(SOURCES_WIDTH)])
                .areas(area);
        let [stock_area, flow_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(charts);

        TimeChart::new(resource.name(), theme)
            .span(&span)
            .range(ResourceHistory::tier_label(tier))
            .value(format!("NOW {}", fmt_compact(now)), theme.good())
            .stat("MIN", fmt_compact(finite(min)), theme.normal())
            .stat("AVG", fmt_compact(avg), theme.normal())
            .stat("MAX", fmt_compact(finite(max)), theme.normal())
            .series(theme.normal(), stock)
            .render(stock_area, buf);

        TimeChart::new("FLOW", theme)
            .span(&span)
            .value(format!("NET {sign}{}/s", fmt_compact(net.abs())), net_style)
            .stat("IN", format!("{}/s", fmt_compact(prod)), theme.good())
            .stat("OUT", format!("{}/s", fmt_compact(cons)), theme.error())
            .series(theme.good(), produced)
            .series(theme.error(), consumed)
            .render(flow_area, buf);

        let sources = history.sources_of(tier, resource);
        SourceBreakdown::new(theme, &sources).render(sources_area, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        let count = ShipResource::iter().count();
        match input {
            Input::ArrowUp => {
                app.ui.resource_selected = (app.ui.resource_selected + count - 1) % count;
            }
            Input::ArrowDown => {
                app.ui.resource_selected = (app.ui.resource_selected + 1) % count;
            }
            Input::Char('-') => {
                app.ui.history_tier =
                    (app.ui.history_tier + 1).min(ResourceHistory::TIER_COUNT - 1);
            }
            Input::Char('+') | Input::Char('=') => {
                app.ui.history_tier = app.ui.history_tier.saturating_sub(1);
            }
            _ => {}
        }
    }
}

fn finite(v: f64) -> f64 {
    if v.is_finite() { v } else { 0.0 }
}

fn span_label(secs: f64) -> String {
    let s = secs.max(0.0) as u64;
    if s < 90 {
        format!("{s}s")
    } else if s < 5400 {
        format!("{}m", s / 60)
    } else if s < 129_600 {
        format!("{}h", s / 3600)
    } else if s < 3_888_000 {
        format!("{}d", s / 86400)
    } else {
        format!("{}mo", s / 2_592_000)
    }
}

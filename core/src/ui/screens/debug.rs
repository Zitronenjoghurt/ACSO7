use crate::app::App;
use crate::input::Input;
use crate::ui::screens::Screen;
use crate::ui::theme::ThemeStyles;
use crate::ui::widgets::chart::fmt_compact;
use crate::ui::widgets::readout::Readout;
use crate::world::ship::resources::ShipResource;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Widget};
use strum::IntoEnumIterator;

const MIN_STEP: f64 = 1.0;
const MAX_STEP: f64 = 1.0e9;

pub struct DebugScreen;

fn selected(app: &App) -> ShipResource {
    ShipResource::iter()
        .nth(app.ui.debug_selected)
        .unwrap_or(ShipResource::Power)
}

impl Screen for DebugScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;
        let perf = &app.performance;
        let resource = selected(app);

        let mut readout = Readout::new("DEBUG", theme).focused(true);
        for (label, timer) in [("UPDATE", &perf.update), ("RENDER", &perf.render)] {
            readout = readout
                .line(Line::from(Span::styled(label, theme.good())))
                .stat("avg", timer.display_average_secs())
                .stat("rate", timer.display_updates_per_sec())
                .stat("budget", timer.display_budget())
                .blank();
        }

        readout
            .line(Line::from(vec![
                Span::styled("GRANT", theme.good()),
                Span::styled(
                    format!(
                        "  ↑↓ PICK · +/- {} · ←→ ×10",
                        fmt_compact(app.ui.debug_grant_step)
                    ),
                    theme.danger(),
                ),
            ]))
            .stat(
                resource.name(),
                format!("{:.0}", app.world.ship.res.get(&resource)),
            )
            .render(area, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        let count = ShipResource::iter().count();
        match input {
            Input::ArrowUp => {
                app.ui.debug_selected = (app.ui.debug_selected + count - 1) % count;
            }
            Input::ArrowDown => {
                app.ui.debug_selected = (app.ui.debug_selected + 1) % count;
            }
            Input::ArrowLeft => {
                app.ui.debug_grant_step = (app.ui.debug_grant_step / 10.0).max(MIN_STEP);
            }
            Input::ArrowRight => {
                app.ui.debug_grant_step = (app.ui.debug_grant_step * 10.0).min(MAX_STEP);
            }
            Input::Char('+') | Input::Char('=') => grant(app, app.ui.debug_grant_step),
            Input::Char('-') => grant(app, -app.ui.debug_grant_step),
            _ => {}
        }
    }
}

fn grant(app: &mut App, delta: f64) {
    let resource = selected(app);
    let amount = (app.world.ship.res.get(&resource) + delta).max(0.0);
    app.world.ship.res.set(resource, amount);
}

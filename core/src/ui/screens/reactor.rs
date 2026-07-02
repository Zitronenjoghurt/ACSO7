use crate::app::App;
use crate::input::Input;
use crate::ui::screens::Screen;
use crate::ui::theme::ThemeStyles;
use crate::ui::widgets::readout::Readout;
use crate::ui::PopupState;
use crate::ui::ShipFocus;
use crate::world::ship::reactor::ReactorMode;
use crate::world::ship::resources::ShipResource;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Widget};

pub struct ReactorScreen;

impl Screen for ReactorScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;
        let ship = &app.world.ship;
        let focused = app.ui.ship_focus == ShipFocus::Systems;

        let mut readout = Readout::new("REACTOR", theme)
            .focused(focused)
            .line(Line::from(vec![
                Span::styled("MODE", theme.normal()),
                Span::styled("   [N] ◂  ▸ [M]", theme.danger()),
            ]));
        for mode in ReactorMode::ALL {
            let active = mode == ship.reactor.mode;
            let marker = if active { "▶ " } else { "  " };
            let style = if active { theme.good() } else { theme.normal() };
            readout = readout.line(Line::from(Span::styled(
                format!("  {marker}{}", mode.label()),
                style,
            )));
        }

        let fuel_for = match ship.reactor.fuel_seconds_remaining(&ship.res) {
            Some(secs) => fuel_label(secs),
            None => "—".to_string(),
        };

        readout
            .blank()
            .bar("HEALTH", ship.reactor.health)
            .stat("FUEL FOR", fuel_for)
            .stat(
                "POWER",
                format!("{:.0} MW", ship.res.get(&ShipResource::Power)),
            )
            .stat("HEAT", format!("{:.0}", ship.res.get(&ShipResource::Heat)))
            .render(area, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        let mode = &mut app.world.ship.reactor.mode;
        match input {
            Input::Char('m') | Input::Char('M') => *mode = mode.next(),
            Input::Char('n') | Input::Char('N') => *mode = mode.prev(),
            _ => {}
        }
    }

    fn help(app: &App) -> Option<PopupState> {
        let theme = &app.config.theme;
        let current = app.world.ship.reactor.mode;
        let mut popup = PopupState::new(theme, "REACTOR MODES");
        for mode in ReactorMode::ALL {
            let active = mode == current;
            let marker = if active { "▶ " } else { "  " };
            let style = if active { theme.good() } else { theme.normal() };
            popup = popup.line(Line::from(Span::styled(
                format!("{marker}{}", mode.label()),
                style,
            )));
            for text in mode.help_lines() {
                popup = popup.text(format!("    {text}"));
            }
            popup = popup.blank();
        }
        Some(popup)
    }
}

fn fuel_label(secs: f64) -> String {
    let s = secs.max(0.0) as u64;
    if s < 90 {
        format!("{s}s")
    } else if s < 5400 {
        format!("{}m", s / 60)
    } else if s < 172_800 {
        format!("{}h", s / 3600)
    } else {
        format!("{}d", s / 86400)
    }
}

use crate::app::App;
use crate::input::Input;
use crate::ui::ShipFocus;
use crate::ui::screens::{Screen, ScreenId};
use crate::ui::theme::ThemeStyles;
use crate::ui::widgets::readout::Readout;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Widget};

pub struct PodsScreen;

impl Screen for PodsScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;
        let pods = &app.world.ship.pods;

        Readout::new("LIFE PODS", theme)
            .focused(app.ui.ship_focus == ShipFocus::Main)
            .stat("POPULATION", pods.pods.len().to_string())
            .bar("INTEGRITY", pods.avg_health())
            .bar("POWER", pods.power_saturation)
            .blank()
            .line(
                Line::from(format!("[ ⏎  VIEW {} COLONISTS  → ]", pods.pods.len()))
                    .style(theme.good())
                    .centered(),
            )
            .render(area, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        match input {
            Input::Enter => app.goto(ScreenId::Colonists),
            Input::Esc | Input::ArrowLeft => app.ui.ship_focus = ShipFocus::Sidebar,
            _ => {}
        }
    }
}

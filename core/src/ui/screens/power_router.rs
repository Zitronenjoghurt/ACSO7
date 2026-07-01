use crate::app::App;
use crate::input::Input;
use crate::ui::ShipFocus;
use crate::ui::screens::Screen;
use crate::ui::widgets::readout::Readout;
use crate::world::ship::resources::ShipResource;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

pub struct PowerRouterScreen;

impl Screen for PowerRouterScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let ship = &app.world.ship;

        Readout::new("POWER GRID", &app.config.theme)
            .focused(app.ui.ship_focus == ShipFocus::Content)
            .stat(
                "GRID",
                format!("{:.0} MW", ship.res.get(&ShipResource::Power)),
            )
            .stat("DEMAND", format!("{:.0} MW", ship.pods.power_demand(1.0)))
            .bar("PODS", ship.pods.power_saturation)
            .render(area, buf);
    }

    fn on_input(_app: &mut App, _input: Input) {}
}

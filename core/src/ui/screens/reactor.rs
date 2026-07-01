use crate::app::App;
use crate::input::Input;
use crate::ui::ShipFocus;
use crate::ui::screens::Screen;
use crate::ui::widgets::readout::Readout;
use crate::world::ship::resources::ShipResource;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

pub struct ReactorScreen;

impl Screen for ReactorScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let ship = &app.world.ship;

        Readout::new("REACTOR", &app.config.theme)
            .focused(app.ui.ship_focus == ShipFocus::Main)
            .stat("MODE", format!("{:?}", ship.reactor.mode))
            .bar("HEALTH", ship.reactor.health)
            .stat(
                "POWER",
                format!("{:.0} MW", ship.res.get(&ShipResource::Power)),
            )
            .stat("HEAT", format!("{:.0}", ship.res.get(&ShipResource::Heat)))
            .render(area, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        if let Input::Esc | Input::ArrowLeft = input {
            app.ui.ship_focus = ShipFocus::Sidebar;
        }
    }
}

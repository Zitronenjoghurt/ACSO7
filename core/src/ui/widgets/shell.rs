use crate::app::App;
use crate::ui::screens::{ScreenId, ShipFocus};
use crate::ui::widgets::log_view::LogView;
use crate::ui::widgets::resources::Resources;
use crate::ui::widgets::sidebar::Sidebar;
use crate::ui::widgets::status_bar::StatusBar;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Widget;

const SIDEBAR_WIDTH: u16 = 32;
const RESOURCE_WIDTH: u16 = 32;
const STATUS_HEIGHT: u16 = 5;
const LOG_HEIGHT: u16 = 5;

pub struct Shell<'a> {
    app: &'a App,
}

impl<'a> Shell<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn render(self, area: Rect, buf: &mut Buffer) -> Rect {
        let theme = &self.app.config.theme;
        let [sidebar, center, resources] = Layout::horizontal([
            Constraint::Length(SIDEBAR_WIDTH),
            Constraint::Fill(1),
            Constraint::Length(RESOURCE_WIDTH),
        ])
        .areas(area);
        let [status, main, log] = Layout::vertical([
            Constraint::Length(STATUS_HEIGHT),
            Constraint::Fill(1),
            Constraint::Length(LOG_HEIGHT),
        ])
        .areas(center);

        Sidebar::new(self.app).render(sidebar, buf);
        Resources::new(
            &self.app.world.ship.res,
            &self.app.world.ship.history,
            theme,
        )
        .focused(self.app.ui.ship_focus == ShipFocus::Resources)
        .showing(self.app.ui.current_screen == ScreenId::Resource)
        .selected(self.app.ui.resource_selected)
        .render(resources, buf);
        StatusBar::new(theme, self.app.world.mission_secs)
            .paused(self.app.paused)
            .render(status, buf);
        LogView::new(&self.app.ui.log, theme).render(log, buf);

        main
    }
}

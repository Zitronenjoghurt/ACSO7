use crate::app::App;
use crate::input::Input;
use crate::ui::screens::{Screen, ScreenId};
use crate::ui::theme::ThemeStyles;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Text, Widget};

pub struct ShipScreen;

impl Screen for ShipScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;

        let [_, name, power, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        Text::from(Line::from(app.world.meta.name.clone()).style(theme.good()))
            .centered()
            .render(name, buf);
        Text::from(Line::from(format!("POWER: {:.1}", app.world.ship.res.power)).style(theme.normal()))
            .centered()
            .render(power, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        match input {
            Input::Char('q') => app.should_quit = true,
            Input::Esc => {
                app.autosave().unwrap();
                app.goto(ScreenId::Title);
            }
            _ => {}
        }
    }

    fn on_enter(app: &mut App) {
        app.ui.saved_worlds = Vec::new();
    }
}

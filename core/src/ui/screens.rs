use crate::ui::theme::ThemeStyles;
use crate::ui::widgets::padded_line::PaddedLine;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Rect, Widget};

pub mod load;
pub mod new_world;
pub mod ship;
pub mod title;

pub trait Screen {
    fn render(app: &crate::app::App, area: Rect, buf: &mut ratatui::buffer::Buffer);
    fn on_input(app: &mut crate::app::App, input: crate::input::Input);
    fn on_enter(_app: &mut crate::app::App) {}
}

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ScreenId {
    #[default]
    Title,
    NewWorld,
    Load,
    Ship,
}

impl ScreenId {
    fn footer(self) -> &'static str {
        match self {
            Self::Title => "[ ↑↓ SELECT │ ⏎ CONFIRM ]",
            Self::NewWorld => "[ TYPE NAME │ ⏎ CREATE │ ESC BACK ]",
            Self::Load => "[ ↑↓ SELECT │ ⏎ LOAD │ ESC BACK ]",
            Self::Ship => "[ ESC SAVE & EXIT │ Q QUIT ]",
        }
    }

    pub fn render(self, app: &crate::app::App, frame: &mut ratatui::Frame) {
        let theme = &app.config.theme;
        let area = frame.area();
        frame.buffer_mut().set_style(area, theme.background());

        let [header, content, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);
        PaddedLine::new("[ Loaded Kernel: Autonomous Colony Ship Operator v7 ]")
            .padding_symbol('═')
            .style(theme.normal())
            .render(header, frame.buffer_mut());
        PaddedLine::new(self.footer())
            .padding_symbol('═')
            .style(theme.normal())
            .render(footer, frame.buffer_mut());

        match self {
            Self::Title => title::TitleScreen::render(app, content, frame.buffer_mut()),
            Self::NewWorld => new_world::NewWorldScreen::render(app, content, frame.buffer_mut()),
            Self::Load => load::LoadScreen::render(app, content, frame.buffer_mut()),
            Self::Ship => ship::ShipScreen::render(app, content, frame.buffer_mut()),
        }
    }

    pub fn on_input(self, app: &mut crate::app::App, input: crate::input::Input) {
        match self {
            Self::Title => title::TitleScreen::on_input(app, input),
            Self::NewWorld => new_world::NewWorldScreen::on_input(app, input),
            Self::Load => load::LoadScreen::on_input(app, input),
            Self::Ship => ship::ShipScreen::on_input(app, input),
        }
    }

    pub fn on_enter(self, app: &mut crate::app::App) {
        match self {
            Self::Title => title::TitleScreen::on_enter(app),
            Self::NewWorld => new_world::NewWorldScreen::on_enter(app),
            Self::Load => load::LoadScreen::on_enter(app),
            Self::Ship => ship::ShipScreen::on_enter(app),
        }
    }
}

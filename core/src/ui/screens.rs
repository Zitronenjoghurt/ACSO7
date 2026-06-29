use crate::ui::theme::ThemeStyles;
use crate::ui::widgets::padded_line::PaddedLine;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Rect, Widget};

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
}

impl ScreenId {
    pub fn render(self, app: &crate::app::App, frame: &mut ratatui::Frame) {
        let theme = &app.store.config.theme;
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
        PaddedLine::new("[ q [QUIT] ]")
            .padding_symbol('═')
            .style(theme.normal())
            .render(footer, frame.buffer_mut());

        match self {
            Self::Title => title::TitleScreen::render(app, content, frame.buffer_mut()),
        }
    }

    pub fn on_input(self, app: &mut crate::app::App, input: crate::input::Input) {
        match self {
            Self::Title => title::TitleScreen::on_input(app, input),
        }
    }

    pub fn on_enter(self, app: &mut crate::app::App) {
        match self {
            Self::Title => title::TitleScreen::on_enter(app),
        }
    }
}

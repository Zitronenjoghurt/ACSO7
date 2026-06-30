use crate::app::App;
use crate::input::Input;
use crate::ui::screens::{Screen, ScreenId};
use crate::ui::theme::ThemeStyles;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Text, Widget};

const MAX_NAME_LEN: usize = 32;

pub struct NewWorldScreen;

impl Screen for NewWorldScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;

        let [_, prompt, field, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        Text::from(Line::from("NAME YOUR SHIP").style(theme.normal()))
            .centered()
            .render(prompt, buf);

        let entry = format!("> {}_", app.ui.new_world_name);
        Text::from(Line::from(entry).style(theme.good()))
            .centered()
            .render(field, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        match input {
            Input::Char(c) => {
                if app.ui.new_world_name.chars().count() < MAX_NAME_LEN {
                    app.ui.new_world_name.push(c);
                }
            }
            Input::Backspace => {
                app.ui.new_world_name.pop();
            }
            Input::Enter => {
                let name = app.ui.new_world_name.trim().to_string();
                if !name.is_empty() {
                    app.create_world(name);
                    app.goto(ScreenId::Ship);
                }
            }
            Input::Esc => app.goto(ScreenId::Title),
            _ => {}
        }
    }

    fn on_enter(app: &mut App) {
        app.ui.new_world_name.clear();
    }
}

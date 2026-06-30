use crate::app::App;
use crate::input::Input;
use crate::ui::screens::{Screen, ScreenId};
use crate::ui::theme::ThemeStyles;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Text, Widget};

pub struct LoadScreen;

impl Screen for LoadScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;
        let selected = app.ui.menu_selected;

        let rows = app.ui.saved_worlds.len().max(1) as u16;
        let [_, list, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(rows),
            Constraint::Fill(1),
        ])
        .areas(area);

        if app.ui.saved_worlds.is_empty() {
            Text::from(Line::from("NO SAVED COLONIES").style(theme.normal()))
                .centered()
                .render(list, buf);
            return;
        }

        let lines: Vec<Line> = app
            .ui
            .saved_worlds
            .iter()
            .enumerate()
            .map(|(i, meta)| {
                let played = meta
                    .last_played
                    .to_zoned(jiff::tz::TimeZone::system())
                    .strftime("%Y-%m-%d %H:%M:%S");
                let label = format!("{}  ({played})", meta.name);
                if i == selected {
                    Line::from(format!("▶ {label} ◀")).style(theme.good())
                } else {
                    Line::from(label).style(theme.normal())
                }
            })
            .collect();
        Text::from(lines).centered().render(list, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        if app.ui.saved_worlds.is_empty() {
            if let Input::Esc = input {
                app.goto(ScreenId::Title);
            }
            return;
        }

        let last = app.ui.saved_worlds.len() - 1;
        match input {
            Input::ArrowUp => {
                app.ui.menu_selected = app.ui.menu_selected.checked_sub(1).unwrap_or(last);
            }
            Input::ArrowDown => {
                app.ui.menu_selected = (app.ui.menu_selected + 1) % app.ui.saved_worlds.len();
            }
            Input::Enter => {
                let id = app.ui.saved_worlds[app.ui.menu_selected].id.clone();
                if app.load_world(&id) {
                    app.goto(ScreenId::Ship);
                }
            }
            Input::Esc => app.goto(ScreenId::Title),
            _ => {}
        }
    }

    fn on_enter(app: &mut App) {
        app.refresh_worlds();
        app.ui.menu_selected = 0;
    }
}

use crate::app::App;
use crate::input::Input;
use crate::ui::screens::{Screen, ScreenId};
use crate::ui::widgets::select_list::SelectList;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Widget;

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

        let labels = app
            .ui
            .saved_worlds
            .iter()
            .map(|meta| {
                let played = meta
                    .last_played
                    .to_zoned(jiff::tz::TimeZone::system())
                    .strftime("%Y-%m-%d %H:%M:%S");
                format!("{}  ({played})", meta.name)
            })
            .collect();
        SelectList::new(theme, labels, selected)
            .empty("NO SAVED COLONIES")
            .render(list, buf);
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
                    app.enter_ship();
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

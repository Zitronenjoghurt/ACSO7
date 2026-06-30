use crate::app::App;
use crate::input::Input;
use crate::ui::effects::FxKey;
use crate::ui::screens::{Screen, ScreenId};
use crate::ui::theme::{Theme, ThemeStyles};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Line, Text, Widget};
use tachyonfx::{fx, Effect, Interpolation};

#[derive(Clone, Copy)]
enum MenuItem {
    Continue,
    Load,
    New,
    Quit,
}

impl MenuItem {
    fn label(self) -> &'static str {
        match self {
            Self::Continue => "CONTINUE",
            Self::Load => "LOAD GAME",
            Self::New => "NEW GAME",
            Self::Quit => "QUIT",
        }
    }
}

fn menu(app: &App) -> Vec<MenuItem> {
    let mut items = Vec::new();
    if !app.ui.saved_worlds.is_empty() {
        items.push(MenuItem::Continue);
        items.push(MenuItem::Load);
    }
    items.push(MenuItem::New);
    items.push(MenuItem::Quit);
    items
}

const LOGO: &[&str] = &[
    "  ▄▄▄▄    ▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄   ▄▄▄▄▄       ▄▄▄▄▄▄▄▄",
    "▄██▀▀██▄ ███▀▀▀▀▀ █████▀▀▀ ▄███████▄     ████████",
    "███  ███ ███       ▀████▄  ███   ███ ██ ██   ▄██▀",
    "███▀▀███ ███         ▀████ ███▄▄▄███ ██▄██  ███  ",
    "███  ███ ▀███████ ███████▀  ▀█████▀   ▀█▀   ███  ",
];

pub struct TitleScreen;

impl Screen for TitleScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;
        let items = menu(app);
        let selected = app.ui.menu_selected;

        let [_, logo, _gap, menu, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(LOGO.len() as u16),
            Constraint::Length(1),
            Constraint::Length(items.len() as u16),
            Constraint::Fill(1),
        ])
        .areas(area);

        let logo_w = LOGO.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let logo_lines: Vec<Line> = LOGO
            .iter()
            .map(|l| Line::from(format!("{l:<logo_w$}")).style(theme.normal()))
            .collect();
        Text::from(logo_lines).centered().render(logo, buf);

        let menu_lines: Vec<Line> = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let label = item.label();
                if i == selected {
                    Line::from(format!("▶ {label} ◀")).style(theme.good())
                } else {
                    Line::from(label.to_string()).style(theme.normal())
                }
            })
            .collect();
        Text::from(menu_lines).centered().render(menu, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        let items = menu(app);
        let last = items.len() - 1;
        match input {
            Input::Char('q') => app.should_quit = true,
            Input::ArrowUp => {
                app.ui.menu_selected = app.ui.menu_selected.checked_sub(1).unwrap_or(last);
            }
            Input::ArrowDown => {
                app.ui.menu_selected = (app.ui.menu_selected + 1) % items.len();
            }
            Input::Enter => match items[app.ui.menu_selected] {
                MenuItem::Continue => {
                    if let Some(meta) = app.ui.saved_worlds.first() {
                        let id = meta.id.clone();
                        if app.load_world(&id) {
                            app.goto(ScreenId::Ship);
                        }
                    }
                }
                MenuItem::Load => app.goto(ScreenId::Load),
                MenuItem::New => app.goto(ScreenId::NewWorld),
                MenuItem::Quit => app.should_quit = true,
            },
            _ => {}
        }
    }

    fn on_enter(app: &mut App) {
        app.refresh_worlds();
        app.ui.menu_selected = 0;
        app.effects
            .add_unique_effect(FxKey::TitleIntro, intro_fx(&app.config.theme));
    }
}

fn intro_fx(theme: &Theme) -> Effect {
    let bg = Color::Rgb(theme.background.r, theme.background.g, theme.background.b);
    fx::parallel(&[
        fx::coalesce((900, Interpolation::QuadOut)),
        fx::fade_from_fg(bg, (700, Interpolation::SineOut)),
    ])
}

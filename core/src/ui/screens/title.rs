use crate::app::App;
use crate::input::Input;
use crate::ui::effects::FxKey;
use crate::ui::screens::Screen;
use crate::ui::theme::{Theme, ThemeStyles};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Line, Text, Widget};
use tachyonfx::{fx, Effect, Interpolation};

pub const MENU: [&str; 2] = ["TO THE SHIP", "QUIT"];

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
        let theme = &app.store.config.theme;
        let selected = app.title_selected;

        let [_, logo, _gap, menu, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(LOGO.len() as u16),
            Constraint::Length(1),
            Constraint::Length(MENU.len() as u16),
            Constraint::Fill(1),
        ])
        .areas(area);

        let logo_w = LOGO.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let logo_lines: Vec<Line> = LOGO
            .iter()
            .map(|l| Line::from(format!("{l:<logo_w$}")).style(theme.normal()))
            .collect();
        Text::from(logo_lines).centered().render(logo, buf);

        let menu_lines: Vec<Line> = MENU
            .iter()
            .enumerate()
            .map(|(i, label)| {
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
        let last = MENU.len() - 1;
        match input {
            Input::ArrowUp => {
                app.title_selected = app.title_selected.checked_sub(1).unwrap_or(last);
            }
            Input::ArrowDown => {
                app.title_selected = (app.title_selected + 1) % MENU.len();
            }
            Input::Enter => match app.title_selected {
                0 => {}
                1 => app.should_quit = true,
                _ => {}
            },
            _ => {}
        }
    }

    fn on_enter(app: &mut App) {
        app.effects
            .add_unique_effect(FxKey::TitleIntro, intro_fx(&app.store.config.theme));
    }
}

fn intro_fx(theme: &Theme) -> Effect {
    let bg = Color::Rgb(theme.background.r, theme.background.g, theme.background.b);
    fx::parallel(&[
        fx::coalesce((900, Interpolation::QuadOut)),
        fx::fade_from_fg(bg, (700, Interpolation::SineOut)),
    ])
}

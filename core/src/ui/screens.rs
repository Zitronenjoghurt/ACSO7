use crate::app::App;
use crate::input::Input;
use crate::ui::widgets::chrome::Chrome;
use crate::ui::widgets::popup::Popup;
use crate::ui::widgets::shell::Shell;
use crate::ui::PopupState;
use crate::world::ship::alert::Alert;
use crate::world::ship::resources::ShipResource;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use strum::IntoEnumIterator;

pub mod colonists;
pub mod debug;
pub mod load;
pub mod new_world;
pub mod pods;
pub mod reactor;
pub mod resource;
pub mod title;

pub trait Screen {
    fn render(app: &crate::app::App, area: Rect, buf: &mut ratatui::buffer::Buffer);
    fn on_input(app: &mut crate::app::App, input: crate::input::Input);
    fn on_enter(_app: &mut crate::app::App) {}
    fn help(_app: &crate::app::App) -> Option<PopupState> {
        None
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ScreenId {
    #[default]
    Title,
    NewWorld,
    Load,
    Reactor,
    Pods,
    Colonists,
    Resource,
    Debug,
}

#[derive(Debug, Clone)]
pub enum VitalCol {
    Gauge { glyph: &'static str, ratio: f64 },
    Text { glyph: &'static str, value: String },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ShipFocus {
    #[default]
    Systems,
    Content,
    Resources,
}

pub const SHIP_SYSTEMS: [ScreenId; 2] = [ScreenId::Reactor, ScreenId::Pods];

impl ScreenId {
    fn in_shell(self) -> bool {
        matches!(
            self,
            Self::Reactor | Self::Pods | Self::Colonists | Self::Resource | Self::Debug
        )
    }

    pub fn shows_system(self) -> bool {
        matches!(self, Self::Reactor | Self::Pods | Self::Colonists)
    }

    pub fn hotkey(self) -> char {
        match self {
            Self::Reactor => 'R',
            Self::Pods => 'P',
            _ => ' ',
        }
    }

    fn from_hotkey(c: char) -> Option<ScreenId> {
        match c.to_ascii_uppercase() {
            'R' => Some(Self::Reactor),
            'P' => Some(Self::Pods),
            _ => None,
        }
    }

    fn footer(self, app: &App) -> &'static str {
        match self {
            Self::Title => "[ ↑↓ SELECT │ ⏎ CONFIRM ]",
            Self::NewWorld => "[ TYPE NAME │ ⏎ CREATE │ ESC BACK ]",
            Self::Load => "[ ↑↓ SELECT │ ⏎ LOAD │ ESC BACK ]",
            Self::Debug => "[ ␣ PAUSE │ ESC CLOSE │ Q QUIT ]",
            Self::Colonists => "[ ↑↓ SCROLL │ ←→ PAGE │ ␣ PAUSE │ ? HELP │ ESC BACK │ Q QUIT ]",
            _ => match app.ui.ship_focus {
                ShipFocus::Systems => {
                    "[ ↑↓ SYSTEM │ →/⇥ RESOURCES │ ␣ PAUSE │ D DEBUG │ ? HELP │ ESC EXIT │ Q QUIT ]"
                }
                ShipFocus::Resources => {
                    "[ ↑↓ RESOURCE │ - + RANGE │ ␣ PAUSE │ D DEBUG │ ? HELP │ ESC EXIT │ Q ]"
                }
                ShipFocus::Content => "[ ↑↓ SCROLL │ ␣ PAUSE │ ? HELP │ ESC BACK │ Q QUIT ]",
            },
        }
    }

    pub fn system_label(self) -> &'static str {
        match self {
            Self::Reactor => "REACTOR",
            Self::Pods => "LIFE PODS",
            _ => "",
        }
    }

    pub fn alerts(self, app: &App) -> Vec<Alert> {
        let ship = &app.world.ship;
        match self {
            Self::Reactor => ship.reactor.alerts(),
            Self::Pods | Self::Colonists => ship.pods.alerts(),
            _ => Vec::new(),
        }
    }

    pub fn vitals(self, app: &App) -> Vec<VitalCol> {
        let ship = &app.world.ship;
        match self {
            Self::Reactor => vec![
                VitalCol::Gauge {
                    glyph: "♥",
                    ratio: ship.reactor.health,
                },
                VitalCol::Gauge {
                    glyph: "☼",
                    ratio: ship.reactor.fusion_saturation,
                },
                VitalCol::Text {
                    glyph: "",
                    value: ship.reactor.mode.label().to_string(),
                },
            ],
            Self::Pods => vec![
                VitalCol::Gauge {
                    glyph: "♥",
                    ratio: ship.pods.avg_health(),
                },
                VitalCol::Text {
                    glyph: "☺",
                    value: ship.pods.pods.len().to_string(),
                },
            ],
            _ => Vec::new(),
        }
    }

    pub fn help(self, app: &App) -> Option<PopupState> {
        match self {
            Self::Reactor => reactor::ReactorScreen::help(app),
            Self::Pods => pods::PodsScreen::help(app),
            Self::Colonists => colonists::ColonistsScreen::help(app),
            Self::Resource => resource::ResourceScreen::help(app),
            _ => None,
        }
    }

    pub fn render(self, app: &App, frame: &mut ratatui::Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();
        let content = Chrome::new(&app.config.theme, self.footer(app))
            .paused(app.paused)
            .render(area, buf);

        if self.in_shell() {
            render_shell(self, app, content, buf);
        } else {
            match self {
                Self::Title => title::TitleScreen::render(app, content, buf),
                Self::NewWorld => new_world::NewWorldScreen::render(app, content, buf),
                Self::Load => load::LoadScreen::render(app, content, buf),
                _ => {}
            }
        }

        if let Some(popup) = &app.ui.popup {
            Popup::new(popup, &app.config.theme).render(area, buf);
        }
    }

    pub fn on_input(self, app: &mut App, input: Input) {
        if self.in_shell() {
            shell_on_input(self, app, input);
            return;
        }

        match self {
            Self::Title => title::TitleScreen::on_input(app, input),
            Self::NewWorld => new_world::NewWorldScreen::on_input(app, input),
            Self::Load => load::LoadScreen::on_input(app, input),
            _ => {}
        }
    }

    pub fn on_enter(self, app: &mut App) {
        match self {
            Self::Title => title::TitleScreen::on_enter(app),
            Self::NewWorld => new_world::NewWorldScreen::on_enter(app),
            Self::Load => load::LoadScreen::on_enter(app),
            Self::Reactor => reactor::ReactorScreen::on_enter(app),
            Self::Pods => pods::PodsScreen::on_enter(app),
            Self::Colonists => colonists::ColonistsScreen::on_enter(app),
            Self::Resource => resource::ResourceScreen::on_enter(app),
            Self::Debug => debug::DebugScreen::on_enter(app),
        }
    }
}

fn render_shell(screen: ScreenId, app: &App, area: Rect, buf: &mut Buffer) {
    let main = Shell::new(app).render(area, buf);

    match screen {
        ScreenId::Reactor => reactor::ReactorScreen::render(app, main, buf),
        ScreenId::Pods => pods::PodsScreen::render(app, main, buf),
        ScreenId::Colonists => colonists::ColonistsScreen::render(app, main, buf),
        ScreenId::Resource => resource::ResourceScreen::render(app, main, buf),
        ScreenId::Debug => debug::DebugScreen::render(app, main, buf),
        _ => {}
    }
}

fn shell_on_input(screen: ScreenId, app: &mut App, input: Input) {
    match input {
        Input::Char(' ') => {
            app.toggle_pause();
            return;
        }
        Input::Char('d') => {
            toggle_debug(app);
            return;
        }
        Input::Char('?') => {
            app.ui.popup = screen.help(app);
            return;
        }
        _ => {}
    }
    if let Input::Char('q') = input {
        app.should_quit = true;
        return;
    }
    if let Input::Char(c) = input
        && let Some(sys) = ScreenId::from_hotkey(c)
    {
        app.ui.system_selected = SHIP_SYSTEMS.iter().position(|&s| s == sys).unwrap_or(0);
        focus_panel(app, ShipFocus::Systems);
        return;
    }
    if let Input::Tab = input {
        let next = match app.ui.ship_focus {
            ShipFocus::Resources => ShipFocus::Systems,
            _ => ShipFocus::Resources,
        };
        focus_panel(app, next);
        return;
    }
    match app.ui.ship_focus {
        ShipFocus::Systems => systems_input(screen, app, input),
        ShipFocus::Resources => resources_input(app, input),
        ShipFocus::Content => content_input(screen, app, input),
    }
}

fn focus_panel(app: &mut App, panel: ShipFocus) {
    app.ui.ship_focus = panel;
    match panel {
        ShipFocus::Systems => {
            let i = app.ui.system_selected.min(SHIP_SYSTEMS.len() - 1);
            app.goto(SHIP_SYSTEMS[i]);
        }
        ShipFocus::Resources => app.goto(ScreenId::Resource),
        ShipFocus::Content => {}
    }
}

fn systems_input(screen: ScreenId, app: &mut App, input: Input) {
    let len = SHIP_SYSTEMS.len();
    match input {
        Input::ArrowUp => select_system(app, len - 1),
        Input::ArrowDown => select_system(app, 1),
        Input::ArrowRight => focus_panel(app, ShipFocus::Resources),
        Input::Esc => exit_to_title(app),
        _ => dispatch(screen, app, input),
    }
}

fn select_system(app: &mut App, step: usize) {
    let len = SHIP_SYSTEMS.len();
    app.ui.system_selected = (app.ui.system_selected + step) % len;
    app.goto(SHIP_SYSTEMS[app.ui.system_selected]);
}

fn resources_input(app: &mut App, input: Input) {
    let count = ShipResource::iter().count();
    match input {
        Input::ArrowUp => select_resource(app, count - 1),
        Input::ArrowDown => select_resource(app, 1),
        Input::ArrowLeft => focus_panel(app, ShipFocus::Systems),
        Input::Esc => exit_to_title(app),
        _ => resource::ResourceScreen::on_input(app, input),
    }
}

fn select_resource(app: &mut App, step: usize) {
    let count = ShipResource::iter().count();
    app.ui.resource_selected = (app.ui.resource_selected + step) % count;
    app.goto(ScreenId::Resource);
}

fn content_input(screen: ScreenId, app: &mut App, input: Input) {
    match input {
        Input::Esc => content_back(screen, app),
        _ => dispatch(screen, app, input),
    }
}

fn dispatch(screen: ScreenId, app: &mut App, input: Input) {
    match screen {
        ScreenId::Reactor => reactor::ReactorScreen::on_input(app, input),
        ScreenId::Pods => pods::PodsScreen::on_input(app, input),
        ScreenId::Colonists => colonists::ColonistsScreen::on_input(app, input),
        ScreenId::Resource => resource::ResourceScreen::on_input(app, input),
        ScreenId::Debug => debug::DebugScreen::on_input(app, input),
        _ => {}
    }
}

fn toggle_debug(app: &mut App) {
    if app.ui.current_screen == ScreenId::Debug {
        focus_panel(app, ShipFocus::Systems);
    } else {
        app.goto(ScreenId::Debug);
        app.ui.ship_focus = ShipFocus::Content;
    }
}

fn content_back(screen: ScreenId, app: &mut App) {
    match screen {
        ScreenId::Colonists => {
            app.goto(ScreenId::Pods);
            app.ui.ship_focus = ShipFocus::Systems;
        }
        ScreenId::Debug => focus_panel(app, ShipFocus::Systems),
        _ => app.ui.ship_focus = ShipFocus::Systems,
    }
}

fn exit_to_title(app: &mut App) {
    app.paused = false;
    app.autosave().unwrap();
    app.goto(ScreenId::Title);
}

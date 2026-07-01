use crate::app::App;
use crate::input::Input;
use crate::ui::widgets::chrome::Chrome;
use crate::ui::widgets::shell::Shell;
use crate::world::ship::resources::ShipResource;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use strum::IntoEnumIterator;

pub mod colonists;
pub mod load;
pub mod new_world;
pub mod pods;
pub mod power_router;
pub mod reactor;
pub mod resource;
pub mod title;

pub trait Screen {
    fn render(app: &crate::app::App, area: Rect, buf: &mut ratatui::buffer::Buffer);
    fn on_input(app: &mut crate::app::App, input: crate::input::Input);
    fn on_enter(_app: &mut crate::app::App) {}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ScreenId {
    #[default]
    Title,
    NewWorld,
    Load,
    Reactor,
    Pods,
    PowerRouter,
    Colonists,
    Resource,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ShipFocus {
    #[default]
    Systems,
    Content,
    Resources,
}

pub const SHIP_SYSTEMS: [ScreenId; 3] = [ScreenId::Reactor, ScreenId::Pods, ScreenId::PowerRouter];

impl ScreenId {
    fn in_shell(self) -> bool {
        matches!(
            self,
            Self::Reactor | Self::Pods | Self::PowerRouter | Self::Colonists | Self::Resource
        )
    }

    pub fn shows_system(self) -> bool {
        matches!(
            self,
            Self::Reactor | Self::Pods | Self::PowerRouter | Self::Colonists
        )
    }

    pub fn hotkey(self) -> char {
        match self {
            Self::Reactor => 'R',
            Self::Pods => 'P',
            Self::PowerRouter => 'G',
            _ => ' ',
        }
    }

    fn from_hotkey(c: char) -> Option<ScreenId> {
        match c.to_ascii_uppercase() {
            'R' => Some(Self::Reactor),
            'P' => Some(Self::Pods),
            'G' => Some(Self::PowerRouter),
            _ => None,
        }
    }

    fn footer(self, app: &App) -> &'static str {
        match self {
            Self::Title => "[ ↑↓ SELECT │ ⏎ CONFIRM ]",
            Self::NewWorld => "[ TYPE NAME │ ⏎ CREATE │ ESC BACK ]",
            Self::Load => "[ ↑↓ SELECT │ ⏎ LOAD │ ESC BACK ]",
            _ => match app.ui.ship_focus {
                ShipFocus::Systems => "[ ↑↓ SYSTEM │ → RESOURCES │ ⏎ OPEN │ ESC EXIT │ Q ]",
                ShipFocus::Resources => "[ ↑↓ RESOURCE │ ← SYSTEMS │ ⏎ OPEN │ ESC EXIT │ Q ]",
                ShipFocus::Content => self.main_footer(),
            },
        }
    }

    fn main_footer(self) -> &'static str {
        match self {
            Self::Colonists => "[ ↑↓ SCROLL │ , . PAGE │ ←→ PANEL │ ESC BACK │ Q QUIT ]",
            Self::Resource => "[ ↑↓ RESOURCE │ - + RANGE │ ←→ PANEL │ ESC BACK │ Q QUIT ]",
            _ => "[ ←→ PANEL │ ⇥ CYCLE │ ESC BACK │ Q QUIT ]",
        }
    }

    pub fn system_label(self) -> &'static str {
        match self {
            Self::Reactor => "REACTOR",
            Self::Pods => "LIFE PODS",
            Self::PowerRouter => "POWER GRID",
            _ => "",
        }
    }

    pub fn vital(self, app: &App) -> (f64, String) {
        let ship = &app.world.ship;
        match self {
            Self::Reactor => (ship.reactor.health, format!("{:?}", ship.reactor.mode)),
            Self::Pods => (ship.pods.avg_health(), ship.pods.pods.len().to_string()),
            Self::PowerRouter => (
                ship.pods.power_saturation,
                format!("{:.0} MW", ship.res.get(&ShipResource::Power)),
            ),
            _ => (0.0, String::new()),
        }
    }

    pub fn render(self, app: &App, frame: &mut ratatui::Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();
        let content = Chrome::new(&app.config.theme, self.footer(app)).render(area, buf);

        if self.in_shell() {
            render_shell(self, app, content, buf);
            return;
        }

        match self {
            Self::Title => title::TitleScreen::render(app, content, buf),
            Self::NewWorld => new_world::NewWorldScreen::render(app, content, buf),
            Self::Load => load::LoadScreen::render(app, content, buf),
            _ => {}
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
            Self::PowerRouter => power_router::PowerRouterScreen::on_enter(app),
            Self::Colonists => colonists::ColonistsScreen::on_enter(app),
            Self::Resource => resource::ResourceScreen::on_enter(app),
        }
    }
}

fn render_shell(screen: ScreenId, app: &App, area: Rect, buf: &mut Buffer) {
    let main = Shell::new(app).render(area, buf);

    match screen {
        ScreenId::Reactor => reactor::ReactorScreen::render(app, main, buf),
        ScreenId::Pods => pods::PodsScreen::render(app, main, buf),
        ScreenId::PowerRouter => power_router::PowerRouterScreen::render(app, main, buf),
        ScreenId::Colonists => colonists::ColonistsScreen::render(app, main, buf),
        ScreenId::Resource => resource::ResourceScreen::render(app, main, buf),
        _ => {}
    }
}

fn shell_on_input(screen: ScreenId, app: &mut App, input: Input) {
    if let Input::Char('q') = input {
        app.should_quit = true;
        return;
    }
    if let Input::Char(c) = input
        && let Some(sys) = ScreenId::from_hotkey(c)
    {
        app.ui.system_selected = SHIP_SYSTEMS.iter().position(|&s| s == sys).unwrap_or(0);
        app.goto(sys);
        app.ui.ship_focus = ShipFocus::Content;
        return;
    }
    if let Input::Tab = input {
        let next = match app.ui.ship_focus {
            ShipFocus::Systems => ShipFocus::Content,
            ShipFocus::Content => ShipFocus::Resources,
            ShipFocus::Resources => ShipFocus::Systems,
        };
        focus_panel(app, next);
        return;
    }
    match app.ui.ship_focus {
        ShipFocus::Systems => systems_input(app, input),
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

fn systems_input(app: &mut App, input: Input) {
    let len = SHIP_SYSTEMS.len();
    match input {
        Input::ArrowUp => select_system(app, len - 1),
        Input::ArrowDown => select_system(app, 1),
        Input::ArrowRight => focus_panel(app, ShipFocus::Resources),
        Input::Enter => app.ui.ship_focus = ShipFocus::Content,
        Input::Esc => exit_to_title(app),
        _ => {}
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
        Input::Enter => app.ui.ship_focus = ShipFocus::Content,
        Input::Esc => exit_to_title(app),
        _ => {}
    }
}

fn select_resource(app: &mut App, step: usize) {
    let count = ShipResource::iter().count();
    app.ui.resource_selected = (app.ui.resource_selected + step) % count;
    app.goto(ScreenId::Resource);
}

fn content_input(screen: ScreenId, app: &mut App, input: Input) {
    match input {
        Input::ArrowLeft => focus_panel(app, ShipFocus::Systems),
        Input::ArrowRight => focus_panel(app, ShipFocus::Resources),
        Input::Esc => content_back(screen, app),
        _ => match screen {
            ScreenId::Reactor => reactor::ReactorScreen::on_input(app, input),
            ScreenId::Pods => pods::PodsScreen::on_input(app, input),
            ScreenId::PowerRouter => power_router::PowerRouterScreen::on_input(app, input),
            ScreenId::Colonists => colonists::ColonistsScreen::on_input(app, input),
            ScreenId::Resource => resource::ResourceScreen::on_input(app, input),
            _ => {}
        },
    }
}

fn content_back(screen: ScreenId, app: &mut App) {
    match screen {
        ScreenId::Colonists => app.goto(ScreenId::Pods),
        ScreenId::Resource => app.ui.ship_focus = ShipFocus::Resources,
        _ => app.ui.ship_focus = ShipFocus::Systems,
    }
}

fn exit_to_title(app: &mut App) {
    app.autosave().unwrap();
    app.goto(ScreenId::Title);
}

use crate::config::Config;
use crate::error::Acos7Result;
use crate::input::Input;
use crate::persistence::{CompressedSerde, PersistenceBackend, WorldStore};
use crate::ui::effects::{Effects, FxKey, pause_fx};
use crate::ui::{ScreenId, ShipFocus, UiState};
use crate::world::{World, WorldId};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub effects: Effects,
    pub last_autosave: jiff::Timestamp,
    pub last_frame: jiff::Timestamp,
    pub last_update: jiff::Timestamp,
    pub paused: bool,
    pub performance: crate::performance::Performance,
    pub should_quit: bool,
    pub ui: UiState,
    pub world: World,
    persistence: Box<dyn PersistenceBackend>,
}

impl App {
    pub fn new(persistence: Box<dyn PersistenceBackend>) -> Acos7Result<Self> {
        let config = match persistence.load(&["config"]) {
            Some(bytes) => Config::from_compressed(&bytes).unwrap_or_default(),
            None => Config::default(),
        };
        let now = jiff::Timestamp::now();
        let mut app = Self {
            config,
            effects: Effects::default(),
            last_autosave: now,
            last_frame: now,
            last_update: now,
            paused: false,
            performance: Default::default(),
            should_quit: false,
            ui: UiState::default(),
            world: World::default(),
            persistence,
        };
        app.ui.current_screen.on_enter(&mut app);
        Ok(app)
    }

    pub fn update(&mut self) {
        self.performance.update.start();

        let now = jiff::Timestamp::now();
        if self.paused {
            self.last_update = now;
            self.performance.update.stop();
            return;
        }
        if self.config.autosave_interval_secs != 0
            && !self.world.meta.id.is_empty()
            && now.duration_since(self.last_autosave).as_secs()
                >= self.config.autosave_interval_secs as i64
        {
            let _ = self.save();
            self.last_autosave = now;
        }

        let dt = now
            .duration_since(self.last_update)
            .as_secs_f64()
            .clamp(0.0, self.config.max_tick_delta_secs);
        if !self.world.meta.id.is_empty() {
            for message in self.world.tick(dt).into_messages() {
                self.ui.log.push(message);
            }
        }
        self.last_update += jiff::SignedDuration::from_secs_f64(dt);

        self.performance.update.stop();
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        self.performance.render.start();

        self.ui.current_screen.render(self, frame);

        let now = jiff::Timestamp::now();
        let elapsed_ms = now.duration_since(self.last_frame).as_millis();
        self.last_frame = now;
        let delta = tachyonfx::Duration::from_millis(elapsed_ms.clamp(0, u32::MAX as i128) as u32);

        let area = frame.area();
        self.effects
            .process_effects(delta, frame.buffer_mut(), area);

        self.performance.render.stop();
    }

    pub fn goto(&mut self, screen: ScreenId) {
        self.ui.current_screen = screen;
        screen.on_enter(self);
    }

    pub fn enter_ship(&mut self) {
        self.ui.ship_focus = ShipFocus::Systems;
        self.ui.system_selected = crate::ui::screens::SHIP_SYSTEMS
            .iter()
            .position(|&s| s == ScreenId::Pods)
            .unwrap_or(0);
        self.goto(ScreenId::Pods);
    }

    pub fn autosave(&mut self) -> Acos7Result<()> {
        if !self.world.meta.id.is_empty() {
            self.world.meta.last_played = jiff::Timestamp::now();
            WorldStore::new(self.persistence.as_ref())
                .save(&self.world, self.config.max_auto_saves)?;
        }
        Ok(())
    }

    pub fn save(&mut self) -> Acos7Result<()> {
        self.persistence
            .save(&["config"], self.config.to_compressed()?);
        self.autosave()?;
        Ok(())
    }

    pub fn refresh_worlds(&mut self) {
        self.ui.saved_worlds = WorldStore::new(self.persistence.as_ref()).list_meta();
    }

    pub fn create_world(&mut self, name: impl Into<String>) {
        self.world = World::new(name);
        self.last_autosave = jiff::Timestamp::now();
    }

    pub fn load_world(&mut self, id: &WorldId) -> bool {
        match WorldStore::new(self.persistence.as_ref()).load_latest(id) {
            Some(world) => {
                self.world = world;
                self.last_autosave = jiff::Timestamp::now();
                true
            }
            None => false,
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.effects
            .add_unique_effect(FxKey::PauseToggle, pause_fx(&self.config.theme));
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn on_input(&mut self, input: Input) {
        if let Some(popup) = self.ui.popup.as_mut() {
            if !popup.on_input(input) {
                self.ui.popup = None;
            }
            return;
        }
        self.ui.current_screen.on_input(self, input);
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.save().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::NullBackend;
    use crate::world::ship::resources::ShipResource;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn renders_resource_screen_with_history() {
        let mut app = App::new(Box::new(NullBackend)).unwrap();
        app.create_world("test");
        app.world.ship.res.set(ShipResource::Deuterium, 10000.0);
        app.world.ship.res.set(ShipResource::Helium3, 10000.0);
        for _ in 0..5 {
            app.world.tick(1.0);
        }

        app.enter_ship();
        app.on_input(Input::Tab);
        assert_eq!(app.ui.ship_focus, ShipFocus::Resources);
        app.on_input(Input::ArrowDown);
        assert_eq!(app.ui.current_screen, ScreenId::Resource);
        app.on_input(Input::Char('-'));

        let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
        let screen = app.ui.current_screen;
        terminal.draw(|frame| screen.render(&app, frame)).unwrap();
    }
}

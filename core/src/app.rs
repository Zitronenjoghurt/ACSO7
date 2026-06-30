use crate::config::Config;
use crate::error::Acos7Result;
use crate::input::Input;
use crate::persistence::{CompressedSerde, PersistenceBackend, WorldStore};
use crate::ui::effects::Effects;
use crate::ui::{ScreenId, UiState};
use crate::world::{World, WorldId};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub effects: Effects,
    pub last_frame: jiff::Timestamp,
    pub last_autosave: jiff::Timestamp,
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
            last_frame: now,
            last_autosave: now,
            should_quit: false,
            ui: UiState::default(),
            world: World::default(),
            persistence,
        };
        app.ui.current_screen.on_enter(&mut app);
        Ok(app)
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

    pub fn autosave(&mut self) -> Acos7Result<()> {
        if !self.world.meta.id.is_empty() {
            self.world.meta.last_played = jiff::Timestamp::now();
            WorldStore::new(self.persistence.as_ref())
                .save(&self.world, self.config.max_auto_saves)?;
        }
        Ok(())
    }

    pub fn update(&mut self) {
        let interval = self.config.autosave_interval_secs;
        if interval == 0 || self.world.meta.id.is_empty() {
            return;
        }
        let now = jiff::Timestamp::now();
        if now.duration_since(self.last_autosave).as_secs() >= interval as i64 {
            let _ = self.autosave();
            self.last_autosave = now;
        }
    }

    pub fn goto(&mut self, screen: ScreenId) {
        self.ui.current_screen = screen;
        screen.on_enter(self);
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        self.ui.current_screen.render(self, frame);

        let now = jiff::Timestamp::now();
        let elapsed_ms = now.duration_since(self.last_frame).as_millis();
        self.last_frame = now;
        let delta = tachyonfx::Duration::from_millis(elapsed_ms.clamp(0, u32::MAX as i128) as u32);

        let area = frame.area();
        self.effects
            .process_effects(delta, frame.buffer_mut(), area);
    }

    pub fn save(&mut self) -> Acos7Result<()> {
        self.persistence
            .save(&["config"], self.config.to_compressed()?);
        self.autosave()?;
        Ok(())
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn on_input(&mut self, input: Input) {
        self.ui.current_screen.on_input(self, input);
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.save().unwrap();
    }
}

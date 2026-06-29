use crate::error::Acos7Result;
use crate::input::Input;
use crate::persistence::CompressedSerde;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub store: crate::store::Store,
    pub title_selected: usize,
    pub effects: crate::ui::effects::Effects,
    last_frame: jiff::Timestamp,
    persistence: Box<dyn crate::persistence::PersistenceBackend>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            store: crate::store::Store::default(),
            should_quit: false,
            title_selected: 0,
            effects: crate::ui::effects::Effects::default(),
            last_frame: jiff::Timestamp::now(),
            persistence: Box::new(crate::persistence::NullBackend),
        };
        app.store.current_screen.on_enter(&mut app);
        app
    }
}

impl App {
    pub fn new(persistence: Box<dyn crate::persistence::PersistenceBackend>) -> Acos7Result<Self> {
        let store = match persistence.load_latest_stamped("base") {
            Some(bytes) => crate::store::Store::from_compressed(&bytes)?,
            None => crate::store::Store::default(),
        };
        let mut app = Self {
            store,
            should_quit: false,
            title_selected: 0,
            effects: crate::ui::effects::Effects::default(),
            last_frame: jiff::Timestamp::now(),
            persistence,
        };
        app.store.current_screen.on_enter(&mut app);
        Ok(app)
    }

    pub fn update(&mut self) {}

    pub fn goto(&mut self, screen: crate::ui::ScreenId) {
        self.store.current_screen = screen;
        screen.on_enter(self);
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        self.store.current_screen.render(self, frame);

        let now = jiff::Timestamp::now();
        let elapsed_ms = now.duration_since(self.last_frame).as_millis();
        self.last_frame = now;
        let delta = tachyonfx::Duration::from_millis(elapsed_ms.clamp(0, u32::MAX as i128) as u32);

        let area = frame.area();
        self.effects
            .process_effects(delta, frame.buffer_mut(), area);
    }

    pub fn save(&mut self) -> Acos7Result<()> {
        let bytes = self.store.to_compressed()?;
        self.persistence
            .stamped_save("base", bytes, self.store.config.max_auto_saves);
        Ok(())
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn on_input(&mut self, input: Input) {
        match input {
            Input::Char('q') => self.should_quit = true,
            _ => self.store.current_screen.on_input(self, input),
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.save().unwrap();
    }
}

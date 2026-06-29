use crate::error::Acos7Result;
use crate::input::Input;
use crate::persistence::CompressedSerde;

#[derive(Debug)]
pub struct App {
    pub store: crate::store::Store,
    persistence: Box<dyn crate::persistence::PersistenceBackend>,
    should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            store: crate::store::Store::default(),
            persistence: Box::new(crate::persistence::NullBackend),
            should_quit: false,
        }
    }
}

impl App {
    pub fn new(persistence: Box<dyn crate::persistence::PersistenceBackend>) -> Acos7Result<Self> {
        let store = match persistence.load_latest_stamped("base") {
            Some(bytes) => crate::store::Store::from_compressed(&bytes)?,
            None => crate::store::Store::default(),
        };
        Ok(Self {
            store,
            persistence,
            should_quit: false,
        })
    }

    pub fn update(&mut self) {}

    pub fn render(&self, frame: &mut ratatui::Frame) {
        crate::ui::render(self, frame);
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
            _ => {}
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.save().unwrap();
    }
}

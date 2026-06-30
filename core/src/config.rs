use crate::ui::theme::Theme;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub autosave_interval_secs: u64,
    pub max_auto_saves: usize,
    pub max_tick_delta_secs: f64,
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            autosave_interval_secs: 60,
            max_auto_saves: 10,
            max_tick_delta_secs: 5.0,
            theme: Theme::default(),
        }
    }
}

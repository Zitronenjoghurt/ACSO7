use crate::ui::theme::{Theme, ThemeStyles};
use ratatui::style::Color;
use tachyonfx::{Effect, EffectManager, Interpolation, fx};

pub type Effects = EffectManager<FxKey>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FxKey {
    #[default]
    TitleIntro,
    PauseToggle,
}

pub fn pause_fx(theme: &Theme) -> Effect {
    let tint = theme.danger().fg.unwrap_or(Color::Reset);
    fx::fade_from_fg(tint, (180, Interpolation::QuadOut))
}

use tachyonfx::EffectManager;

pub type Effects = EffectManager<FxKey>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FxKey {
    #[default]
    TitleIntro,
}

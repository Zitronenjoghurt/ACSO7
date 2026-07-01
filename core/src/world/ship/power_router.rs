#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PowerRoute {
    Pods,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PowerRouter {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertLevel {
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alert {
    pub level: AlertLevel,
    pub label: &'static str,
}

impl Alert {
    pub fn warning(label: &'static str) -> Self {
        Self {
            level: AlertLevel::Warning,
            label,
        }
    }

    pub fn critical(label: &'static str) -> Self {
        Self {
            level: AlertLevel::Critical,
            label,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Input {
    Char(char),
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Backspace,
    Enter,
    Esc,
    Tab,
    Home,
    End,
}

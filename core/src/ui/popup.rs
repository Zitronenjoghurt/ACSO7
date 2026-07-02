use crate::input::Input;
use crate::ui::theme::{Theme, ThemeStyles};
use ratatui::prelude::{Line, Span};

const PAGE: usize = 5;

#[derive(Debug)]
pub struct PopupState {
    pub title: String,
    pub lines: Vec<Line<'static>>,
    pub scroll: usize,
    theme: Theme,
}

impl PopupState {
    pub fn new(theme: &Theme, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            lines: Vec::new(),
            scroll: 0,
            theme: *theme,
        }
    }

    pub fn section(mut self, text: impl Into<String>) -> Self {
        self.lines
            .push(Line::from(Span::styled(text.into(), self.theme.good())));
        self
    }

    pub fn stat(mut self, label: &str, value: impl Into<String>) -> Self {
        self.lines.push(Line::from(vec![
            Span::styled(format!("  {label:<10}"), self.theme.normal()),
            Span::styled(value.into(), self.theme.good()),
        ]));
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.lines
            .push(Line::from(Span::styled(text.into(), self.theme.normal())));
        self
    }

    pub fn blank(mut self) -> Self {
        self.lines.push(Line::from(""));
        self
    }

    pub fn line(mut self, line: Line<'static>) -> Self {
        self.lines.push(line);
        self
    }

    pub fn on_input(&mut self, input: Input) -> bool {
        match input {
            Input::ArrowUp => self.scroll = self.scroll.saturating_sub(1),
            Input::ArrowDown => self.scroll = (self.scroll + 1).min(self.max_scroll()),
            Input::Char(',') => self.scroll = self.scroll.saturating_sub(PAGE),
            Input::Char('.') => self.scroll = (self.scroll + PAGE).min(self.max_scroll()),
            Input::Home => self.scroll = 0,
            Input::End => self.scroll = self.max_scroll(),
            _ => return false,
        }
        true
    }

    fn max_scroll(&self) -> usize {
        self.lines.len().saturating_sub(1)
    }
}

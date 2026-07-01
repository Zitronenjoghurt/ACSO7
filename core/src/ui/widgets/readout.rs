use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Text, Widget};

const CARD_WIDTH: u16 = 54;
const LABEL_WIDTH: usize = 11;
const BAR_WIDTH: usize = 16;

enum Row<'a> {
    Stat { label: &'a str, value: String },
    Bar { label: &'a str, ratio: f64 },
    Free(Line<'a>),
}

pub struct Readout<'a> {
    title: &'a str,
    theme: &'a Theme,
    focused: bool,
    rows: Vec<Row<'a>>,
}

impl<'a> Readout<'a> {
    pub fn new(title: &'a str, theme: &'a Theme) -> Self {
        Self {
            title,
            theme,
            focused: true,
            rows: Vec::new(),
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn stat(mut self, label: &'a str, value: impl Into<String>) -> Self {
        self.rows.push(Row::Stat {
            label,
            value: value.into(),
        });
        self
    }

    pub fn bar(mut self, label: &'a str, ratio: f64) -> Self {
        self.rows.push(Row::Bar { label, ratio });
        self
    }

    pub fn blank(mut self) -> Self {
        self.rows.push(Row::Free(Line::from("")));
        self
    }

    pub fn line(mut self, line: Line<'a>) -> Self {
        self.rows.push(Row::Free(line));
        self
    }
}

impl Widget for Readout<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = Panel::new(self.title, self.theme)
            .focused(self.focused)
            .fixed(CARD_WIDTH, self.rows.len() as u16)
            .render(area, buf);
        let lines: Vec<Line> = self.rows.iter().map(|row| row.line(self.theme)).collect();
        Text::from(lines).render(inner, buf);
    }
}

impl<'a> Row<'a> {
    fn line(&self, theme: &Theme) -> Line<'a> {
        match self {
            Row::Stat { label, value } => Line::from(vec![
                Span::styled(format!("{label:<LABEL_WIDTH$}"), theme.normal()),
                Span::styled(value.clone(), theme.good()),
            ]),
            Row::Bar { label, ratio } => {
                let r = ratio.clamp(0.0, 1.0);
                let filled = (r * BAR_WIDTH as f64).round() as usize;
                let bar = format!("{}{}", "█".repeat(filled), "░".repeat(BAR_WIDTH - filled));
                Line::from(vec![
                    Span::styled(format!("{label:<LABEL_WIDTH$}"), theme.normal()),
                    Span::styled(bar, theme.saturation(r)),
                    Span::styled(format!(" {:>3.0}%", r * 100.0), theme.normal()),
                ])
            }
            Row::Free(line) => line.clone(),
        }
    }
}

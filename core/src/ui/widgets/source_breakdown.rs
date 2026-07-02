use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::chart::fmt_compact;
use crate::ui::widgets::panel::Panel;
use crate::world::ship::resources::history::SourceFlow;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Text, Widget};
use ratatui::widgets::Padding;

pub struct SourceBreakdown<'a> {
    theme: &'a Theme,
    sources: &'a [SourceFlow],
}

impl<'a> SourceBreakdown<'a> {
    pub fn new(theme: &'a Theme, sources: &'a [SourceFlow]) -> Self {
        Self { theme, sources }
    }
}

impl Widget for SourceBreakdown<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = Panel::new("SOURCES", self.theme)
            .padding(Padding::new(1, 1, 1, 0))
            .render(area, buf);

        let lines: Vec<Line> = if self.sources.is_empty() {
            vec![Line::from("no recorded flow").style(self.theme.normal())]
        } else {
            self.sources
                .iter()
                .flat_map(|s| {
                    let mut flow: Vec<Span> = Vec::new();
                    if s.produced > 0.0 {
                        flow.push(Span::styled(
                            format!("  +{}/s", fmt_compact(s.produced)),
                            self.theme.good(),
                        ));
                    }
                    if s.consumed > 0.0 {
                        flow.push(Span::styled(
                            format!("  -{}/s", fmt_compact(s.consumed)),
                            self.theme.error(),
                        ));
                    }
                    [
                        Line::from(Span::styled(s.source.label(), self.theme.normal())),
                        Line::from(flow),
                        Line::default(),
                    ]
                })
                .collect()
        };
        Text::from(lines).render(inner, buf);
    }
}

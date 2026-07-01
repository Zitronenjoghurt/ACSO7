use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Widget};
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};

pub struct Series {
    pub style: Style,
    pub data: Vec<(f64, f64)>,
}

pub struct TimeChart<'a> {
    title: &'a str,
    theme: &'a Theme,
    series: Vec<Series>,
    span: String,
    range: Option<String>,
    value: Option<(String, Style)>,
    stats: Vec<(&'a str, String, Style)>,
}

impl<'a> TimeChart<'a> {
    pub fn new(title: &'a str, theme: &'a Theme) -> Self {
        Self {
            title,
            theme,
            series: Vec::new(),
            span: String::new(),
            range: None,
            value: None,
            stats: Vec::new(),
        }
    }

    pub fn stat(mut self, label: &'a str, value: impl Into<String>, style: Style) -> Self {
        self.stats.push((label, value.into(), style));
        self
    }

    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    pub fn span(mut self, span: impl Into<String>) -> Self {
        self.span = span.into();
        self
    }

    pub fn value(mut self, value: impl Into<String>, style: Style) -> Self {
        self.value = Some((value.into(), style));
        self
    }

    pub fn series(mut self, style: Style, data: Vec<(f64, f64)>) -> Self {
        self.series.push(Series { style, data });
        self
    }

    fn bounds(&self) -> ([f64; 2], [f64; 2]) {
        let x_max = self
            .series
            .iter()
            .map(|s| s.data.len())
            .max()
            .unwrap_or(0)
            .saturating_sub(1)
            .max(1) as f64;

        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for point in self.series.iter().flat_map(|s| &s.data) {
            y_min = y_min.min(point.1);
            y_max = y_max.max(point.1);
        }
        if !y_min.is_finite() || !y_max.is_finite() {
            y_min = 0.0;
            y_max = 1.0;
        }
        if (y_max - y_min).abs() < f64::EPSILON {
            y_max = y_min + 1.0;
        }
        ([0.0, x_max], [y_min, y_max])
    }
}

impl Widget for TimeChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut panel = Panel::new(self.title, self.theme).focused(false);
        if let Some(range) = &self.range {
            panel = panel.left(Line::from(format!(" {range} ")).style(self.theme.normal()));
        }
        if let Some((text, style)) = &self.value {
            panel = panel.right(Line::from(format!(" {text} ")).style(*style));
        }
        if !self.stats.is_empty() {
            let mut spans: Vec<Span> = vec![Span::styled(" ", self.theme.normal())];
            for (i, (label, value, style)) in self.stats.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::styled("  ", self.theme.normal()));
                }
                spans.push(Span::styled(format!("{label} "), self.theme.normal()));
                spans.push(Span::styled(value.clone(), *style));
            }
            spans.push(Span::styled(" ", self.theme.normal()));
            panel = panel.bottom(Line::from(spans));
        }
        let inner = panel.render(area, buf);

        let ([x_lo, x_hi], [y_lo, y_hi]) = self.bounds();

        let datasets: Vec<Dataset> = self
            .series
            .iter()
            .map(|s| {
                Dataset::default()
                    .marker(Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(s.style)
                    .data(&s.data)
            })
            .collect();

        let y_label = |v: f64| Span::styled(fmt_compact(v), self.theme.normal());
        let x_left = if self.span.is_empty() {
            String::new()
        } else {
            format!("-{}", self.span)
        };
        Chart::new(datasets)
            .x_axis(
                Axis::default()
                    .style(self.theme.normal())
                    .labels(vec![
                        Line::from(Span::styled(x_left, self.theme.normal())),
                        Line::from(Span::styled("now", self.theme.good())).right_aligned(),
                    ])
                    .bounds([x_lo, x_hi]),
            )
            .y_axis(
                Axis::default()
                    .style(self.theme.normal())
                    .labels(vec![
                        y_label(y_lo),
                        y_label((y_lo + y_hi) / 2.0),
                        y_label(y_hi),
                    ])
                    .bounds([y_lo, y_hi]),
            )
            .render(inner, buf);
    }
}

pub fn fmt_compact(v: f64) -> String {
    let a = v.abs();
    if a >= 1.0e9 {
        format!("{:.1}G", v / 1.0e9)
    } else if a >= 1.0e6 {
        format!("{:.1}M", v / 1.0e6)
    } else if a >= 1.0e3 {
        format!("{:.1}k", v / 1.0e3)
    } else if a >= 10.0 {
        format!("{v:.0}")
    } else {
        format!("{v:.1}")
    }
}

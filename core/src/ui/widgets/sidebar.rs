use crate::app::App;
use crate::ui::screens::{SHIP_SYSTEMS, ScreenId, ShipFocus, VitalCol};
use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::alerts::Alerts;
use crate::ui::widgets::panel::Panel;
use crate::world::ship::alert::AlertLevel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Text, Widget};
use ratatui::style::Modifier;
use ratatui::text::Span;
use ratatui::widgets::Padding;

const COL_INDENT: &str = "      ";

pub struct Sidebar<'a> {
    app: &'a App,
}

impl<'a> Sidebar<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for Sidebar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.config.theme;
        let focus = self.app.ui.ship_focus;
        let selected = self.app.ui.system_selected.min(SHIP_SYSTEMS.len() - 1);
        let show = self.app.ui.current_screen.shows_system();

        let inner = Panel::new(&self.app.world.meta.name, theme)
            .focused(focus == ShipFocus::Systems)
            .padding(Padding::new(1, 1, 1, 0))
            .render(area, buf);

        let mut lines: Vec<Line> = Vec::new();
        for (i, sys) in SHIP_SYSTEMS.iter().enumerate() {
            let alerts = sys.alerts(self.app);
            let widget = Alerts::new(&alerts, theme).indent(5);
            lines.push(label_row(
                *sys,
                i == selected && show,
                focus,
                widget.top_level(),
                theme,
            ));
            lines.push(vitals_row(&sys.vitals(self.app), theme));
            lines.extend(widget.lines());
        }
        Text::from(lines).render(inner, buf);
    }
}

fn label_row<'a>(
    sys: ScreenId,
    active: bool,
    focus: ShipFocus,
    alert: Option<AlertLevel>,
    theme: &Theme,
) -> Line<'a> {
    let focused = active && focus == ShipFocus::Systems;
    let (marker, marker_style) = if !active {
        (" ", theme.normal())
    } else if focused {
        ("▶", theme.good())
    } else {
        ("▶", theme.normal().add_modifier(Modifier::DIM))
    };
    let label_style = match alert {
        Some(AlertLevel::Critical) => theme.error().add_modifier(Modifier::BOLD),
        Some(AlertLevel::Warning) => theme.danger().add_modifier(Modifier::BOLD),
        None if focused => theme.good(),
        None => theme.normal(),
    };
    Line::from(vec![
        Span::styled(format!("[{}] ", sys.hotkey()), theme.danger()),
        Span::styled(marker, marker_style),
        Span::styled(format!(" {}", sys.system_label()), label_style),
    ])
}

fn vitals_row<'a>(cols: &[VitalCol], theme: &Theme) -> Line<'a> {
    let mut spans = vec![Span::raw(COL_INDENT)];
    for (i, col) in cols.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        match col {
            VitalCol::Gauge { glyph, ratio } => {
                let r = ratio.clamp(0.0, 1.0);
                spans.push(Span::styled(
                    format!("{glyph}{:>3.0}%", r * 100.0),
                    theme.saturation(r),
                ));
            }
            VitalCol::Text { glyph, value } => {
                spans.push(Span::styled(format!("{glyph}{value}"), theme.normal()));
            }
        }
    }
    Line::from(spans)
}

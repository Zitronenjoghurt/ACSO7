use crate::app::App;
use crate::ui::screens::{SHIP_SYSTEMS, ScreenId, ShipFocus};
use crate::ui::theme::{Theme, ThemeStyles};
use crate::ui::widgets::panel::Panel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Text, Widget};
use ratatui::style::Modifier;
use ratatui::text::Span;
use ratatui::widgets::Padding;

pub struct Sidebar<'a> {
    app: &'a App,
    active: ScreenId,
}

impl<'a> Sidebar<'a> {
    pub fn new(app: &'a App, active: ScreenId) -> Self {
        Self { app, active }
    }
}

impl Widget for Sidebar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.config.theme;
        let focus = self.app.ui.ship_focus;

        let inner = Panel::new(&self.app.world.meta.name, theme)
            .focused(focus == ShipFocus::Sidebar)
            .padding(Padding::new(1, 1, 1, 0))
            .render(area, buf);

        let lines: Vec<Line> = SHIP_SYSTEMS
            .iter()
            .map(|sys| {
                let (ratio, value) = sys.vital(self.app);
                system_row(*sys, ratio, &value, *sys == self.active, focus, theme)
            })
            .collect();
        Text::from(lines).render(inner, buf);
    }
}

fn system_row<'a>(
    sys: ScreenId,
    ratio: f64,
    value: &str,
    selected: bool,
    focus: ShipFocus,
    theme: &Theme,
) -> Line<'a> {
    let r = ratio.clamp(0.0, 1.0);
    let active_here = selected && focus == ShipFocus::Sidebar;
    let (marker, marker_style) = match (selected, focus) {
        (true, ShipFocus::Sidebar) => ("▶", theme.good()),
        (true, ShipFocus::Main) => ("▶", theme.normal().add_modifier(Modifier::DIM)),
        _ => (" ", theme.normal()),
    };
    let label_style = if active_here {
        theme.good()
    } else {
        theme.normal()
    };
    Line::from(vec![
        Span::styled(format!("[{}] ", sys.hotkey()), theme.danger()),
        Span::styled(marker, marker_style),
        Span::styled(format!("{:<11}", sys.system_label()), label_style),
        Span::styled(format!("{:>3.0}% ", r * 100.0), theme.saturation(r)),
        Span::styled(format!("{value:>7}"), theme.normal()),
    ])
}

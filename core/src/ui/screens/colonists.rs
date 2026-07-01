use crate::app::App;
use crate::input::Input;
use crate::ui::screens::Screen;
use crate::ui::theme::ThemeStyles;
use crate::ui::widgets::alerts::Alerts;
use crate::ui::widgets::panel::Panel;
use crate::world::colonist::Sex;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Text, Widget};
use ratatui::style::Modifier;

const PAGE: usize = 10;

pub struct ColonistsScreen;

impl Screen for ColonistsScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;
        let pods = &app.world.ship.pods;
        let total = pods.pods.len();
        let avg = pods.avg_health();

        let integrity =
            Line::from(format!(" INTEGRITY {:>3.0}% ", avg * 100.0)).style(theme.saturation(avg));
        let inner = Panel::new(&format!("COLONISTS ({total})"), theme)
            .right(integrity)
            .render(area, buf);
        let height = inner.height as usize;
        if height == 0 || total == 0 {
            return;
        }

        let alerts = pods.alerts();
        let banner = Alerts::new(&alerts, theme).lines();
        let list_height = height.saturating_sub(banner.len());
        if list_height == 0 {
            Text::from(banner).render(inner, buf);
            return;
        }

        let selected = app.ui.colonist_selected.min(total - 1);
        let offset = if total <= list_height {
            0
        } else {
            selected
                .saturating_sub(list_height / 2)
                .min(total - list_height)
        };
        let failing = pods.life_support_failing();

        let mut lines = banner;
        lines.extend(
            pods.pods[offset..(offset + list_height).min(total)]
                .iter()
                .enumerate()
                .map(|(i, pod)| {
                    let idx = offset + i;
                    let c = &pod.colonist;
                    let sex = match c.sex {
                        Sex::Female => "F",
                        Sex::Male => "M",
                    };
                    let marker = if idx == selected { "▶ " } else { "  " };
                    let info = format!(
                        "{marker}{:>4}  {:<28} {}  {:>3}",
                        idx + 1,
                        c.full_name(),
                        sex,
                        c.age
                    );
                    let info_style = if idx == selected {
                        theme.good()
                    } else {
                        theme.normal()
                    };
                    let mut spans = vec![
                        Span::styled(info, info_style),
                        Span::styled(
                            format!("   {:>3.0}%", pod.health * 100.0),
                            theme.saturation(pod.health),
                        ),
                    ];
                    if failing {
                        spans.push(Span::styled(
                            " ‼",
                            theme
                                .error()
                                .add_modifier(Modifier::REVERSED | Modifier::BOLD),
                        ));
                    }
                    Line::from(spans)
                }),
        );
        Text::from(lines).render(inner, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        let total = app.world.ship.pods.pods.len();
        if total == 0 {
            return;
        }
        let last = total - 1;
        match input {
            Input::ArrowUp => {
                app.ui.colonist_selected = app.ui.colonist_selected.checked_sub(1).unwrap_or(last);
            }
            Input::ArrowDown => {
                app.ui.colonist_selected = (app.ui.colonist_selected + 1) % total;
            }
            Input::Char(',') => {
                app.ui.colonist_selected = app.ui.colonist_selected.saturating_sub(PAGE);
            }
            Input::Char('.') => {
                app.ui.colonist_selected = (app.ui.colonist_selected + PAGE).min(last);
            }
            Input::Home => {
                app.ui.colonist_selected = 0;
            }
            Input::End => {
                app.ui.colonist_selected = last;
            }
            _ => {}
        }
    }

    fn on_enter(app: &mut App) {
        app.ui.colonist_selected = 0;
    }
}

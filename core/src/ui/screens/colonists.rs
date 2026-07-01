use crate::app::App;
use crate::input::Input;
use crate::ui::screens::{Screen, ScreenId};
use crate::ui::theme::ThemeStyles;
use crate::ui::widgets::panel::Panel;
use crate::world::colonist::Sex;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Text, Widget};

const PAGE: usize = 10;

pub struct ColonistsScreen;

impl Screen for ColonistsScreen {
    fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let theme = &app.config.theme;
        let pods = &app.world.ship.pods;
        let total = pods.pods.len();

        let inner = Panel::new(&format!("COLONISTS ({total})"), theme).render(area, buf);
        let height = inner.height as usize;
        if height == 0 || total == 0 {
            return;
        }

        let selected = app.ui.colonist_selected.min(total - 1);
        let offset = if total <= height {
            0
        } else {
            selected.saturating_sub(height / 2).min(total - height)
        };

        let lines: Vec<Line> = pods.pods[offset..(offset + height).min(total)]
            .iter()
            .enumerate()
            .map(|(i, pod)| {
                let idx = offset + i;
                let c = &pod.colonist;
                let sex = match c.sex {
                    Sex::Female => "F",
                    Sex::Male => "M",
                };
                let row = format!(
                    "{:>4}  {:<28} {}  {:>3}",
                    idx + 1,
                    c.full_name(),
                    sex,
                    c.age
                );
                if idx == selected {
                    Line::from(format!("▶ {row}")).style(theme.good())
                } else {
                    Line::from(format!("  {row}")).style(theme.normal())
                }
            })
            .collect();
        Text::from(lines).render(inner, buf);
    }

    fn on_input(app: &mut App, input: Input) {
        let total = app.world.ship.pods.pods.len();
        if total == 0 {
            if let Input::Esc = input {
                app.goto(ScreenId::Pods);
            }
            return;
        }
        let last = total - 1;
        match input {
            Input::Esc => app.goto(ScreenId::Pods),
            Input::ArrowUp => {
                app.ui.colonist_selected = app.ui.colonist_selected.checked_sub(1).unwrap_or(last);
            }
            Input::ArrowDown => {
                app.ui.colonist_selected = (app.ui.colonist_selected + 1) % total;
            }
            Input::ArrowLeft => {
                app.ui.colonist_selected = app.ui.colonist_selected.saturating_sub(PAGE);
            }
            Input::ArrowRight => {
                app.ui.colonist_selected = (app.ui.colonist_selected + PAGE).min(last);
            }
            _ => {}
        }
    }

    fn on_enter(app: &mut App) {
        app.ui.colonist_selected = 0;
    }
}

use crate::app::App;
use crate::ui::widgets::padded_line::PaddedLine;
use ratatui::layout::Constraint;
use ratatui::prelude::{Layout, Widget};

mod widgets;

pub fn render(app: &App, frame: &mut ratatui::Frame) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    PaddedLine::new("[ q [QUIT] ]")
        .padding_symbol('═')
        .render(footer, frame.buffer_mut());
}

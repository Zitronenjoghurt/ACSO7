use acso7::app::App;
use acso7::input::Input;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use std::time::Duration;

mod persistence;

fn main() -> std::io::Result<()> {
    let persistence = persistence::FileBackend::new(std::env::current_dir()?.join("acso7"));
    let mut app = App::new(Box::new(persistence)).unwrap();
    let mut terminal = ratatui::init();

    while !app.should_quit() {
        terminal.draw(|frame| app.render(frame))?;

        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && let Some(input) = map_input(key.code)
        {
            app.on_input(input);
        }

        app.update();
    }

    ratatui::restore();
    Ok(())
}

fn map_input(code: KeyCode) -> Option<Input> {
    Some(match code {
        KeyCode::Char(c) => Input::Char(c),
        KeyCode::Up => Input::ArrowUp,
        KeyCode::Down => Input::ArrowDown,
        KeyCode::Left => Input::ArrowLeft,
        KeyCode::Right => Input::ArrowRight,
        KeyCode::Backspace => Input::Backspace,
        KeyCode::Enter => Input::Enter,
        KeyCode::Esc => Input::Esc,
        KeyCode::Tab => Input::Tab,
        _ => return None,
    })
}

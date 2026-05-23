use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::data::app::App;

pub async fn event(app: &mut App) {
    if event::poll(Duration::from_millis(200)).unwrap()
        && let Event::Key(key) = event::read().unwrap()
    {
        match key.code {
            KeyCode::Esc => app.is_quit = true,
            _ => {}
        }
    }
}

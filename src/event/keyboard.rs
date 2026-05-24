use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::data::app::App;

pub async fn event(app: &mut App) {
    if event::poll(Duration::from_millis(200)).unwrap()
        && let Event::Key(key) = event::read().unwrap()
    {
        match key.code {
            KeyCode::Esc => app.is_quit = true,
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Enter => {
                if app.input.is_empty() {
                } else {
                    let input = app.input.clone();
                    let mut llm = app.llm_handle.clone();
                    app.input.clear();
                    tokio::spawn(async move {
                        llm.chat(input).await.unwrap();
                    });
                }
            }
            _ => {}
        }
    }
}

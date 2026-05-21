use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::data::app::{App, Mode};

pub async fn event(app: &mut App) {
    if event::poll(Duration::from_millis(200)).unwrap()
        && let Event::Key(key) = event::read().unwrap()
    {
        match app.mode {
            Mode::Input => {
                if let KeyCode::Char(c) = key.code {
                    app.input.push(c);
                }

                if key.code == KeyCode::Backspace {
                    app.input.pop();
                }

                if key.code == KeyCode::Enter && app.mode == Mode::Input {
                    let input = app.input.clone();
                    if !input.is_empty() {
                        return;
                    }
                    app.input.clear();

                    let mut llm_handle = app.llm_handle.clone();
                    tokio::spawn(async move {
                        let _ = llm_handle.chat(input).await;
                    });
                }
            }
            Mode::Normal => {
                if key.code == KeyCode::Esc {
                    app.exit = true
                }
            }
            Mode::CurrChat => {
                if key.code == KeyCode::Char('j') {
                    app.next_current_chat_select();
                }
                if key.code == KeyCode::Char('k') {
                    app.prew_current_chat_select();
                }
            }
            Mode::ScrollChat => {
                if key.code == KeyCode::Char('j') {
                    app.scroll_j();
                }
                if key.code == KeyCode::Char('k') {
                    app.scroll_k();
                }
            }
            Mode::Code => {}
        }

        if key.code == KeyCode::Tab {
            match app.mode {
                Mode::Input => app.mode = Mode::CurrChat,
                Mode::Normal => app.mode = Mode::Input,
                Mode::CurrChat => app.mode = Mode::ScrollChat,
                Mode::ScrollChat => {
                    app.mode = Mode::Code;
                    app.scroll = 0;
                }
                Mode::Code => app.mode = Mode::Normal,
            }
        }
    }
}

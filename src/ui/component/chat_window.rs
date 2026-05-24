use core::panic;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Spacing},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders},
};

use crate::data::app::App;

pub fn draw(frame: &mut Frame, rect: Rect, app: &App) -> anyhow::Result<()> {
    let mut chat_text = String::new();
    let mut think_text = String::new();
    let mut have_messages = false;
    let mut messages = Vec::new();

    match app.llm_handle.history.read() {
        Ok(rw) => {
            if !rw.is_empty() {
                have_messages = true;
                messages = rw.clone();
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!()
        }
    }

    if have_messages {
        for mes in messages {
            chat_text.push_str(mes.chat_content.unwrap().as_str());
            think_text.push_str(mes.think_content.unwrap().as_str());
        }
    } else {
        chat_text.push_str("没有任何聊天");
    }

    Ok(())
}

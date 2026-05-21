use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::data::app::App;

pub fn draw(frame: &mut Frame, rect: Rect, app: &App) {
    let i = app.llm_handle.history.read().unwrap().len()
        - 1
        - app.histoy_chat_list_state.selected().unwrap_or(0);
    let chat_text = app.llm_handle.history.read().unwrap()[i]
        .clone()
        .content
        .unwrap();
    let thinks_text = app.llm_handle.history.read().unwrap()[i]
        .clone()
        .think_content
        .unwrap_or_default();

    let text = if !thinks_text.is_empty() {
        format!("[{}]\n\n{}", thinks_text, chat_text)
    } else {
        chat_text
    };
    let chat = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .scroll((app.scroll, 0))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("聊天窗口")
                .title_style(Style::default().fg(Color::Magenta))
                .border_style(Style::default().fg(Color::LightMagenta)),
        );

    frame.render_widget(chat, rect);
}

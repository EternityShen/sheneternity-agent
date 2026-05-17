use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::data::app::App;

pub fn draw(frame: &mut Frame, rect: Rect, app: &App) {
    let chat_list_widget = Paragraph::new("Chat_List_0").block(
        Block::default()
            .borders(Borders::ALL)
            .title("历史聊天列表")
            .title_style(Style::default().fg(Color::Blue))
            .border_style(Style::default().fg(Color::Blue)),
    );

    frame.render_widget(chat_list_widget, rect);
}

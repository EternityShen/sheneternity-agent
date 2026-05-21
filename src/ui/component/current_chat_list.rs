use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::data::app::App;

pub fn draw(frame: &mut Frame, rect: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .llm_handle
        .history
        .read()
        .unwrap()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, message)| {
            let is_selected = app.histoy_chat_list_state.selected() == Some(i);

            let style = if is_selected {
                Style::default().fg(Color::Gray).bg(Color::Magenta)
            } else {
                Style::default().fg(Color::White)
            };

            let span = if is_selected {
                Span::styled(format!("> {}", message.content.clone().unwrap()), style)
            } else {
                Span::styled(format!("{}", message.content.clone().unwrap()), style)
            };

            ListItem::new(Line::from(span))
        })
        .collect();

    let chat_list_widget = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("当前聊天列表")
            .title_style(Style::default().fg(Color::Blue))
            .border_style(Style::default().fg(Color::Blue)),
    );

    frame.render_widget(chat_list_widget, rect);
}

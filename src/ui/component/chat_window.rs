use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::data::app::{App, Mode};

pub fn draw(frame: &mut Frame, rect: Rect, app: &App) {
    match app.mode {
        Mode::Input => {
            let chat_text = app.llm_handle.llm_output.read().unwrap().clone();

            let chat = Paragraph::new(chat_text).wrap(Wrap::default()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("聊天窗口")
                    .title_style(Style::default().fg(Color::Magenta))
                    .border_style(Style::default().fg(Color::LightMagenta)),
            );

            frame.render_widget(chat, rect);
        }
        Mode::Normal => {
            let i = app.llm_handle.history.read().unwrap().len()
                - 1
                - app.current_chat_list_state.selected().unwrap_or(0);
            let chat_text = app.llm_handle.history.read().unwrap()[i].clone();

            let chat = Paragraph::new(chat_text.content)
                .wrap(Wrap::default())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("聊天窗口")
                        .title_style(Style::default().fg(Color::Magenta))
                        .border_style(Style::default().fg(Color::LightMagenta)),
                );

            frame.render_widget(chat, rect);
        }
        Mode::CurrChat => {
            let i = app.llm_handle.history.read().unwrap().len()
                - 1
                - app.current_chat_list_state.selected().unwrap_or(0);
            let chat_text = app.llm_handle.history.read().unwrap()[i].clone();

            let chat = Paragraph::new(chat_text.content)
                .wrap(Wrap::default())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("聊天窗口")
                        .title_style(Style::default().fg(Color::Magenta))
                        .border_style(Style::default().fg(Color::LightMagenta)),
                );

            frame.render_widget(chat, rect);
        }
        Mode::ScrollChat => {
            let i = app.llm_handle.history.read().unwrap().len()
                - 1
                - app.current_chat_list_state.selected().unwrap_or(0);
            let chat_text = app.llm_handle.history.read().unwrap()[i].clone();

            let chat = Paragraph::new(chat_text.content)
                .wrap(Wrap::default())
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
    }
}

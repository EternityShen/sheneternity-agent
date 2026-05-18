use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::data::app::{App, Mode};

pub fn draw(frame: &mut Frame, rect: Rect, app: &mut App) {
    let cursor = if app.cursor {
        app.cursor = false;
        String::from("|")
    } else {
        app.cursor = true;
        String::new()
    };
    match app.mode {
        Mode::Input => {
            let input_bar_widget = Paragraph::new(Text::styled(
                format!("{}{}", app.input, cursor),
                Style::default().fg(Color::White),
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta))
                    .title("输入框")
                    .title_style(Style::default().fg(Color::LightMagenta)),
            );

            frame.render_widget(input_bar_widget, rect);
        }
        _ => {
            let input_bar_widget = Paragraph::new(Text::styled(
                app.input.to_string(),
                Style::default().fg(Color::White),
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta))
                    .title("输入框")
                    .title_style(Style::default().fg(Color::LightMagenta)),
            );

            frame.render_widget(input_bar_widget, rect);
        }
    }
}

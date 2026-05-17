use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::data::app::App;

pub fn draw(frame: &mut Frame, rect: Rect, app: &App) {
    let input_bar_widget = Paragraph::new(Text::styled(
        format!("{}", app.input),
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

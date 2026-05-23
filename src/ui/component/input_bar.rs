use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::data::app::App;

pub fn draw(frame: &mut Frame, rect: Rect, app: &mut App) -> anyhow::Result<()> {
    let text = Paragraph::new(Text::styled(
        app.input.clone(),
        Style::default().fg(Color::White),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(text, rect);
    Ok(())
}

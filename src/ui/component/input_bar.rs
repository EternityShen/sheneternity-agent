use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::data::app::App;

pub fn draw(frame: &mut Frame, rect: Rect, app: &mut App) -> anyhow::Result<()> {
    let text = if app.curror {
        app.curror = !app.curror;
        Paragraph::new(Text::styled(
            format!("{}$", app.input.clone()),
            Style::default().fg(Color::White),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .merge_borders(MergeStrategy::Exact),
        )
    } else {
        app.curror = !app.curror;
        Paragraph::new(Text::styled(
            app.input.clone(),
            Style::default().fg(Color::White),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .merge_borders(MergeStrategy::Exact),
        )
    };

    frame.render_widget(text, rect);
    Ok(())
}

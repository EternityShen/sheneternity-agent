use ratatui::{
    Frame,
    layout::{HorizontalAlignment, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::data::app::{App, Mode};

pub fn draw(frame: &mut Frame, rect: Rect, app: &App) {
    let mode_widget = match app.mode {
        Mode::Input => Paragraph::new(Text::styled("Input", Style::default().fg(Color::White)))
            .alignment(HorizontalAlignment::Center),
        Mode::Normal => Paragraph::new(Text::styled("Normal", Style::default().fg(Color::White)))
            .alignment(HorizontalAlignment::Center),
        Mode::CurrChat => {
            Paragraph::new(Text::styled("CurrChat", Style::default().fg(Color::White)))
                .alignment(HorizontalAlignment::Center)
        }
        Mode::ScrollChat => {
            Paragraph::new(Text::styled("Scroll", Style::default().fg(Color::White)))
                .alignment(HorizontalAlignment::Center)
        }
        Mode::Code => Paragraph::new(Text::styled("Code", Style::default().fg(Color::White)))
            .alignment(HorizontalAlignment::Center),
    };

    let mode_widget = mode_widget.block(
        Block::default()
            .borders(Borders::ALL)
            .title("模式")
            .title_style(Color::Blue)
            .border_style(Color::Blue),
    );

    frame.render_widget(mode_widget, rect);
}

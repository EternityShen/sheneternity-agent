use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders},
};

use crate::{
    data::app::App,
    ui::component::{app_mode, chat_window, current_chat_list, histoy_caht_list, input_bar},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks_v = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .spacing(0)
    .split(frame.area());

    let chunks_h = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .spacing(1)
    .split(chunks_v[1]);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("ShenEternity-Agent")
        .title_style(Style::default().fg(Color::Blue))
        .border_style(Style::default().fg(Color::LightBlue))
        .border_set(symbols::border::ROUNDED)
        .title_alignment(ratatui::layout::HorizontalAlignment::Center);

    frame.render_widget(block, frame.area());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightYellow))
        .border_set(symbols::border::DOUBLE);

    frame.render_widget(block, chunks_h[1]);

    let chunks_v = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .spacing(0)
    .split(chunks_h[1]);

    let chunks_h = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .spacing(1)
    .split(chunks_v[1]);

    let chunks_h = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
        .spacing(1)
        .split(chunks_h[1]);

    let left_chunks_v = Layout::vertical([
        Constraint::Length(3),
        Constraint::Percentage(60),
        Constraint::Percentage(40),
    ])
    .spacing(0)
    .split(chunks_h[0]);

    let left_top = left_chunks_v[0];
    let left_mid = left_chunks_v[1];
    let left_bottom = left_chunks_v[2];

    app_mode::draw(frame, left_top, app);
    current_chat_list::draw(frame, left_mid, app);
    histoy_caht_list::draw(frame, left_bottom, app);

    let right_chunks_v =
        Layout::vertical([Constraint::Min(30), Constraint::Length(app.input_bar_size)])
            .spacing(0)
            .split(chunks_h[1]);

    let right_top = right_chunks_v[0];
    let right_bottom = right_chunks_v[1];

    chat_window::draw(frame, right_top, app);
    input_bar::draw(frame, right_bottom, app);
}

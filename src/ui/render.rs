use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{model::artist, ui::View, ui::build::{build_library_panel, build_queue_panel, build_now_playing}};
use crate::app::{App};
use crate::model::identifier::{ArtistIdentifier, AlbumIdentifier};
use crate::db;

pub fn render_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Length(3)])
        .split(f.area());

    // Top: library + queue
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[0]);

    let lib_panel = build_library_panel(app);
    let queue_panel = build_queue_panel(app);

    f.render_stateful_widget(
        List::new(lib_panel.items).block(Block::default().borders(Borders::ALL).title(lib_panel.title)).highlight_symbol("▶ "),
        top_chunks[0],
        &mut app.list_state.clone(),
    );

    f.render_widget(
        List::new(queue_panel.items).block(Block::default().borders(Borders::ALL).title("Queue")),
        top_chunks[1],
    );

    // Bottom: now playing
    let now_playing_panel = build_now_playing(app);

    let now_playing_widget = Paragraph::new(now_playing_panel.text)
        .block(Block::default().borders(Borders::ALL).title("Now Playing"));

    f.render_widget(now_playing_widget, chunks[1]);
}

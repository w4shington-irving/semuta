use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem},
};

use crate::ui::View;
use crate::app::{App, NowPlaying};

pub fn render_ui(f: &mut Frame, app: &mut App) {
    

    let size = f.area();

    let items = match app.view {
        View::Artists => app.artists.iter().map(|a| a.name.clone()).collect(),
        View::Albums { .. } => app.albums.iter().map(|a| a.title.clone()).collect(),
        View::Tracks { .. } => {
            let mut sorted_tracks: Vec<_> = app.tracks.clone();
            sorted_tracks.sort_by_key(|t| t.track_number.unwrap_or(0));
            sorted_tracks
                .iter()
                .map(|t| {
                    let number = t.track_number.unwrap_or(0); // fallback if missing
                    format!("{}. {}", number, t.title)
                })
                .collect::<Vec<String>>()

            },
        _ => Vec::new(),
    };

    let list_items: Vec<ListItem> =
        items.into_iter().map(ListItem::new).collect();

    let title = match app.view {
        View::Artists => "Artists",
        View::Albums { .. } => "Albums",
        View::Tracks { .. } => "Tracks",
    };

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, size, &mut app.list_state);
}


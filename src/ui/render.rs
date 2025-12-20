use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{model::artist, ui::View};
use crate::app::{App};
use crate::model::identifier::{ArtistIdentifier, AlbumIdentifier};
use crate::db;

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
        View::Artists => " Library ",
        View::Albums { .. } => {
            let artist_name = db::get_artist(&ArtistIdentifier::Id(app.selected.artist_id.unwrap())).expect("Failed to get artist").name;
            &format!(" {} ", artist_name)
        },
        View::Tracks { .. } => {
            let artist_name = db::get_artist(&ArtistIdentifier::Id(app.selected.artist_id.unwrap())).expect("Failed to get artist").name;
            let album_name = db::get_album(&AlbumIdentifier::Id(app.selected.album_id.unwrap())).expect("Failed to get album").title;
            &format!(" {}, {} ", artist_name, album_name)
        }
    };

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, size, &mut app.list_state);
}


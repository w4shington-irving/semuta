use crate::app::{App};
use crate::ui::{View};
use crate::db;
use crate::model::identifier::{ArtistIdentifier, AlbumIdentifier};
use ratatui::widgets::ListItem;

/// Prebuilt UI content for library panel
pub struct LibraryPanel<'a> {
    pub items: Vec<ListItem<'a>>,
    pub title: String,
}

/// Prebuilt UI content for queue panel
pub struct QueuePanel<'a> {
    pub items: Vec<ListItem<'a>>,
}

pub struct NowPlayingPanel {
    pub text: String,
}

/// Build the Now Playing panel
pub fn build_now_playing(app: &App) -> NowPlayingPanel {
    let text = if let Some(track) = &app.now_playing.track {
        let number = track.track_number.unwrap_or(0);
        format!("▶ {}. {}", number, track.title)
    } else {
        "No track playing".to_string()
    };

    NowPlayingPanel { text }
}

/// Build everything needed for rendering
pub fn build_library_panel(app: &App) -> LibraryPanel {
    let items: Vec<String> = match app.view {
        View::Artists => app.artists.iter().map(|a| a.name.clone()).collect(),
        View::Albums { .. } => app.albums.iter().map(|a| a.title.clone()).collect(),
        View::Tracks { .. } => {
            let mut sorted_tracks = app.tracks.clone();
            sorted_tracks.sort_by_key(|t| t.track_number.unwrap_or(0));
            sorted_tracks
                .iter()
                .map(|t| {
                    let number = t.track_number.unwrap_or(0);
                    format!("{}. {}", number, t.title)
                })
                .collect()
        }
        _ => Vec::new(),
    };

    let list_items: Vec<ListItem> = items.into_iter().map(ListItem::new).collect();

    let title = match app.view {
        View::Artists => " Library ".to_string(),
        View::Albums { .. } => {
            let artist_name = db::get_artist(&ArtistIdentifier::Id(app.selected.artist_id.unwrap()))
                .expect("Failed to get artist")
                .name;
            format!(" {} ", artist_name)
        }
        View::Tracks { .. } => {
            let artist_name = db::get_artist(&ArtistIdentifier::Id(app.selected.artist_id.unwrap()))
                .expect("Failed to get artist")
                .name;
            let album_name = db::get_album(&AlbumIdentifier::Id(app.selected.album_id.unwrap()))
                .expect("Failed to get album")
                .title;
            format!(" {}, {} ", artist_name, album_name)
        }
    };

    LibraryPanel {
        items: list_items,
        title,
    }
}

/// Build the queue panel
pub fn build_queue_panel(app: &App) -> QueuePanel {
    let items: Vec<ListItem> = app.queue
        .iter()
        .map(|t| {
            let label = format!("{} ({} - {})", t.title, t.artist_name, t.album_name);
            ListItem::new(label)
        })
        .collect();

    QueuePanel { items }
}
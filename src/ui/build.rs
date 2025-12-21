use crate::app::{App};
use crate::ui::{View};
use crate::db;
use crate::model::identifier::{ArtistIdentifier, AlbumIdentifier};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, ListItem},
};

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
    pub title: Paragraph<'static>,
    pub progress: Option<Gauge<'static>>,
}

pub fn build_now_playing(app: &App) -> NowPlayingPanel {
    if let Some(now) = &app.now_playing {
        // Track info
        let track_title = &now.track.title;

        let total = now.track.duration_secs.max(1); // avoid div by 0
        let elapsed = now.position.min(total as u64);
        let ratio = elapsed as f64 / total as f64;

        // Title paragraph (centered)
        let title = Paragraph::new(track_title.clone())
            .block(Block::default().borders(Borders::NONE))
            .alignment(ratatui::layout::Alignment::Center);

        // Bottom progress row
        let play_symbol = if now.paused { "⏸" } else { "▶" };
        let elapsed_str = format!("{:02}:{:02}", elapsed / 60, elapsed % 60);
        let total_str = format!("{:02}:{:02}", total / 60, total % 60);
        let label = format!("{} {} / {}", play_symbol, elapsed_str, total_str);

        let progress = Gauge::default()
            .ratio(ratio)
            .label(label)
            .block(Block::default().borders(Borders::NONE))
            .gauge_style(Style::default().fg(Color::LightGreen));

        NowPlayingPanel {
            title,
            progress: Some(progress),
        }
    } else {
        // No track playing: only show a placeholder
        let title = Paragraph::new("No track playing")
            .block(Block::default().borders(Borders::NONE))
            .alignment(ratatui::layout::Alignment::Center);

        NowPlayingPanel {
            title,
            progress: None, // no gauge
        }
    }
}

/// Build everything needed for rendering
pub fn build_library_panel(app: &'_ App) -> LibraryPanel<'_> {
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
        },
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
pub fn build_queue_panel(app: &'_ App) -> QueuePanel<'_> {
    let items: Vec<ListItem> = app.queue
        .iter()
        .map(|t| {
            let label = format!("{} ({} - {})", t.title, t.artist_name, t.album_name);
            ListItem::new(label)
        })
        .collect();

    QueuePanel { items }
}
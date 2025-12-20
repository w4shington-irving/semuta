use std::process::exit;

use crossterm::event::KeyCode;

use crate::app::App;
use crate::db::get_album;
use crate::model::{album::Album, identifier::AlbumIdentifier};
use crate::ui::View;
use crate::db;

pub fn handle_key(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Up => app.list_state.select_previous(),
        KeyCode::Down => app.list_state.select_next(),

        KeyCode::Enter => match app.view {
            View::Artists => {
                if let Some(i) = app.list_state.selected() {
                    let artist_id = app.artists[i].id;
                    app.load_albums(artist_id);
                }
            }
            View::Albums { .. } => {
                if let Some(i) = app.list_state.selected() {
                    let album_id = app.albums[i].id;
                    app.load_tracks(album_id);
                }
            }
            View::Tracks { .. } => {
                if let Some(i) = app.list_state.selected() {
                    let track = app.tracks[i].clone();
                    app.now_playing = Some(track);
                    // future: audio::play(track)
                }
            }
        },

        KeyCode::Backspace => match app.view {
            View::Tracks { album_id } => {
                // go back to albums
                let album = db::get_album(&AlbumIdentifier::Id(album_id));
                if let Ok(album) = album {
                    app.load_albums(album.artist_id);
                }
            }
            View::Albums { .. } => app.load_artists(),
            View::Artists => {}
        },
        KeyCode::Char('q') => exit(0),
        _ => {}
    }
}

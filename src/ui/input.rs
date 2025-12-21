use std::process::exit;

use crossterm::event::KeyCode;

use crate::app::App;
use crate::model::identifier::AlbumIdentifier;
use crate::ui::View;
use crate::db;

pub struct Selected {
    pub artist_id: Option<i64>,
    pub artist_index: Option<usize>,
    pub album_id: Option<i64>,
    pub album_index: Option<usize>,
}

impl Selected {
    pub fn new() -> Self {
        Self {
            artist_id: None,
            artist_index: None,
            album_id: None,
            album_index: None,
        }
    }
}


pub fn handle_key(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Up => app.list_state.select_previous(),
        KeyCode::Down => app.list_state.select_next(),
        KeyCode::Right => match app.view {
            View::Artists => {
                if let Some(i) = app.list_state.selected() {
                    app.selected.artist_id = Some(app.artists[i].id);
                    app.selected.artist_index = Some(i);
                    let artist_id = app.artists[i].id;
                    app.load_albums(artist_id);
                }
            }
            View::Albums { .. } => {
                if let Some(i) = app.list_state.selected() {
                    app.selected.album_id = Some(app.albums[i].id);
                    app.selected.album_index = Some(i);
                    let album_id = app.albums[i].id;
                    app.load_tracks(album_id);
                }
            }
            View::Tracks { album_id: _ } => {}
        },
        KeyCode::Enter => match app.view {
            View::Artists => {
                if let Some(i) = app.list_state.selected() {
                    app.selected.artist_id = Some(app.artists[i].id);
                    app.selected.artist_index = Some(i);
                    let artist_id = app.artists[i].id;
                    app.load_albums(artist_id);
                }
            }
            View::Albums { .. } => {
                if let Some(i) = app.list_state.selected() {
                    app.selected.album_id = Some(app.albums[i].id);
                    app.selected.album_index = Some(i);
                    let album_id = app.albums[i].id;
                    app.load_tracks(album_id);
                    let mut tracks = app.tracks.clone();
                    app.enqueue(&mut tracks);

                }
            }
            View::Tracks { .. } => {
                if let Some(i) = app.list_state.selected() {
                    app.stop();
                    let track = app.tracks[i].clone();
                    app.play(track);
                    
                }
            }
        },
        
        KeyCode::Char(' ') => app.toggle_play_pause(),
        KeyCode::F(10) => app.play_next(),
        KeyCode::F(8) => app.play_previous(),
        KeyCode::Char('p') => app.pause(),
        KeyCode::Char('r') => app.resume(),
        KeyCode::Char('s') => app.stop(),

        KeyCode::Backspace => match app.view {
            View::Tracks { album_id } => {
                // go back to albums
                let album = db::get_album(&AlbumIdentifier::Id(album_id));
                if let Ok(album) = album {
                    app.load_albums(album.artist_id);
                }
                app.list_state.select(app.selected.album_index);
            }

            View::Albums { .. } => {
                app.load_artists();
                app.list_state.select(app.selected.artist_index);
            }

            View::Artists => {}
        },
        KeyCode::Char('q') => exit(0),
        _ => {}
    }
}



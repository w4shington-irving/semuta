use std::time::Duration;
use crate::ui::View;
use crate::model::{track::Track, album::Album, artist::Artist, identifier::{ArtistIdentifier, AlbumIdentifier}};
use crate::db;


#[derive(Debug, Clone)]
pub struct NowPlaying {
    pub track: Track,
    pub album: Album,
    pub artist: Artist,
    pub duration: Duration,
    pub playing: bool,
}

use ratatui::widgets::ListState;

pub struct App {
    pub view: View,

    pub artists: Vec<Artist>,
    pub albums: Vec<Album>,
    pub tracks: Vec<Track>,

    pub list_state: ListState,

    pub now_playing: Option<Track>,
}

impl App {
    pub fn new() -> Self {
        Self {
            view: View::Artists,
            artists: Vec::new(),
            albums: Vec::new(),
            tracks: Vec::new(),
            list_state: ListState::default(),
            now_playing: None,
        }
    }
    
    pub fn load_artists(&mut self) {
        self.artists = db::get_artists().expect("Failed to get artists");
        self.list_state.select(Some(0));
        self.view = View::Artists;
    }

    pub fn load_albums(&mut self, artist_id: i64) {
        self.albums = db::get_albums(&ArtistIdentifier::Id(artist_id)).expect("Failed to get albums");
        self.list_state.select(Some(0));
        self.view = View::Albums { artist_id };
    }

    pub fn load_tracks(&mut self, album_id: i64) {
        self.tracks = db::get_tracks(&AlbumIdentifier::Id(album_id)).expect("Failed to get tracks");
        self.list_state.select(Some(0));
        self.view = View::Tracks { album_id };
    }
}


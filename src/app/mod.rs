use std::time::Duration;
use crate::ui::{View, input::Selected};
use crate::model::{track::Track, album::Album, artist::Artist, identifier::{ArtistIdentifier, AlbumIdentifier}};
use crate::db::{self, tracks};
use crate::audio;
use rodio::queue;
use rodio::{OutputStream, Sink, stream::OutputStreamBuilder};
use std::sync::Arc;
use audio::Player;


pub struct NowPlaying {
        // controls playback
    pub track: Option<Track>,
    pub position: u64,
}

impl NowPlaying {
    pub fn new() -> Self {
        Self {
            track: None,
            position: 0,
        }
    }
}

use ratatui::widgets::ListState;

pub struct App {
    pub view: View,

    pub selected: Selected,
    pub artists: Vec<Artist>,
    pub albums: Vec<Album>,
    pub tracks: Vec<Track>,

    pub queue: Vec<Track>,
    pub list_state: ListState,

    pub player: Player,
    pub now_playing: NowPlaying,
    was_playing: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            view: View::Artists,
            selected: Selected::new(),
            artists: Vec::new(),
            albums: Vec::new(),
            tracks: Vec::new(),
            list_state: ListState::default(),
            player: Player::new().expect("Failed to create audio player"),
            queue: Vec::new(),
            now_playing: NowPlaying::new(),
            was_playing: false,
        }
    }
    
    pub fn update(&mut self) {

        let is_playing_now =
            !self.player.is_idle() && !self.player.is_paused();

        // Detect: playing → finished
        if self.was_playing && self.player.is_idle() {
            self.on_track_finished();
        }

        self.was_playing = is_playing_now;

    }

    fn on_track_finished(&mut self) {
        // For now, just clear state
        self.now_playing.track = None;

        if !self.queue.is_empty() {
            // Play next track
            self.play_next();
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

    pub fn enqueue(&mut self, tracks: &mut Vec<Track>) {
        if self.queue.is_empty() && !tracks.is_empty() {
            let track = tracks.remove(0);
            self.play(track.clone());
        }
        
        self.queue.append(&mut tracks.clone());
        
    }
        
    pub fn play(&mut self, track: Track) {

        self.player
            .play_track(&track.path)
            .expect("playback failed");

        self.now_playing.track = Some(track);
        self.was_playing = true;
        
    }

    pub fn pause(&mut self) {
        self.player.pause();
    }

    pub fn resume(&mut self) {
        self.player.resume();
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.queue.clear();
        self.now_playing.track = None;
    }

    pub fn play_next(&mut self) {
        if !self.queue.is_empty() {
            let track = self.queue.remove(0);
            self.play(track);
        } else {
            self.stop();
        }
    }

    pub fn toggle_play_pause(&mut self) {
        match self.player.is_paused() {
            true => self.player.resume(),
            false => self.player.pause(), 
        }
    }
}



use crate::ui::{View, input::Selected};
use crate::model::{track::Track, album::Album, artist::Artist, identifier::{ArtistIdentifier, AlbumIdentifier}};
use crate::db;
use crate::audio;
use audio::Player;



pub struct NowPlaying {
        // controls playback
    pub track: Track,
    pub position: u64,
    pub paused: bool,
}

impl NowPlaying {
    pub fn new(track: Track) -> Self {
        Self {
            track: track,
            position: 0,
            paused: false,
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

    pub previous_tracks: Vec<Track>,
    pub queue: Vec<Track>,
    pub list_state: ListState,

    pub player: Player,
    pub now_playing:  Option<NowPlaying>,
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
            previous_tracks: Vec::new(),
            now_playing: None,
            was_playing: false,
        }
    }

    pub fn tick(&mut self, dt: u64) {
        if let Some(now) = &mut self.now_playing {
            if !now.paused {
                let total = now.track.duration_secs;
                now.position = (now.position + dt).min(total as u64);
            }
        }
    }

    pub fn update(&mut self) {

        self.tick(1);

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
        self.previous_tracks.push(self.now_playing.as_ref().unwrap().track.clone());
        self.now_playing = None;

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

        self.now_playing = Some(NowPlaying::new(track));
        self.was_playing = true;
        
    }

    pub fn pause(&mut self) {
        self.player.pause();
        self.now_playing.as_mut().unwrap().paused = true;
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();
        self.previous_tracks.clear();
    }

    pub fn resume(&mut self) {
        self.player.resume();
        self.now_playing.as_mut().unwrap().paused = false;
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.clear_queue();
        self.now_playing = None;
    }

    pub fn play_previous(&mut self) {
        if !self.previous_tracks.is_empty() {
            let track = self.previous_tracks.pop().unwrap();
            self.play(track);
        } 
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
            true => self.resume(),
            false => self.pause(), 
        }
    }
}

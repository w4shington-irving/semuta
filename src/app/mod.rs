use std::time::Duration;
use crate::ui::{View, input::Selected};
use crate::model::{track::Track, album::Album, artist::Artist, identifier::{ArtistIdentifier, AlbumIdentifier}};
use crate::db::{self, tracks};
use crate::audio;
use rodio::queue;
use rodio::{OutputStream, Sink, stream::OutputStreamBuilder};
use std::sync::Arc;


pub struct NowPlaying {
    pub sink: Arc<Sink>,        // controls playback
    pub track: Track,
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

    pub output: OutputStream,
    pub now_playing: Option<NowPlaying>,
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
            output: OutputStreamBuilder::open_default_stream().expect("Failed to open audio output stream"),
            queue: Vec::new(),
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

    pub fn enqueue(&mut self, track: Track) {
        self.queue.push(track);
    }

    pub fn play(&mut self, track: Track) {
        
        let sink = audio::play_track(&self.output, &track.path);
        self.now_playing = Some(NowPlaying {
            sink: sink.unwrap(),
            track: track,
        });
    }

    pub fn pause(&mut self) {
        if let Some(np) = &self.now_playing {
            np.sink.pause();
        }
    }

    pub fn resume(&mut self) {
        if let Some(np) = &self.now_playing {
            np.sink.play();
        }
    }

    pub fn stop(&mut self) {
        if let Some(np) = &self.now_playing {
            np.sink.stop();
            self.now_playing = None;
        }
    }

    pub fn toggle_play_pause(&mut self) {
        if let Some(np) = &self.now_playing {
            if np.sink.is_paused() {
                np.sink.play();   // resume if paused
            } else {
                np.sink.pause();  // pause if playing
            }
        }
    }
}

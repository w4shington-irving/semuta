
use std::fs::File;
use std::io::BufReader;
use rodio::{OutputStream, Decoder, Sink};
use std::sync::Arc;
use crate::model::track::{self, Track};
use std::path::Path;
use std::time::{Instant, Duration};


use rodio::OutputStreamBuilder;

pub struct Player {
    _stream: OutputStream, // must live as long as audio plays
    sink: Sink,
}

impl Player {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(stream.mixer());
        Ok(Self {
            _stream: stream,
            sink,
        })
    }

    

    /// Non-blocking, single-track playback
    pub fn play_track<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.sink.stop();

        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        self.sink.append(source);
        self.sink.play();

        Ok(())
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn is_idle(&self) -> bool {
        self.sink.empty()
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }
}




pub fn play_queue(
    stream: &OutputStream,
    queue: &Vec<Track>,
) -> Result<Arc<Sink>, Box<dyn std::error::Error>> {
    // Create a sink connected to the existing audio device
    let sink = Sink::connect_new(stream.mixer());

    // Open audio file
    for track in queue {
        let file = File::open(&track.path)?;
        let source = Decoder::new(BufReader::new(file))?;
        sink.append(source);
    }
    Ok(Arc::new(sink))
}





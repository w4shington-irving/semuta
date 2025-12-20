
use std::fs::File;
use std::io::BufReader;
use rodio::{OutputStream, Decoder, Sink};
use std::sync::Arc;


pub fn play_track(
    stream: &OutputStream,
    path: &str,
) -> Result<Arc<Sink>, Box<dyn std::error::Error>> {
    // Create a sink connected to the existing audio device
    let sink = Sink::connect_new(stream.mixer());

    // Open audio file
    let file = File::open(path)?;
    let source = Decoder::new(BufReader::new(file))?;

    // Start playback
    sink.append(source);

    Ok(Arc::new(sink))
}




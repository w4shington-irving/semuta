use std::{fs::File, io::BufReader};
use rodio::{Decoder, OutputStream, source::Source};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let library_path = "~/Music/";
    
    // Get an output stream handle to the default physical sound device.
    // Note that the playback stops when the stream_handle is dropped.
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("song.flac").unwrap());
    // Note that the playback stops when the sink is dropped
    let sink = rodio::play(&stream_handle.mixer(), file).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    sink.sleep_until_end();

    Ok(())
}

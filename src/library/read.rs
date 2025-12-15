
use lofty::{file::AudioFile, prelude::TaggedFileExt, tag::Accessor};
use crate::model::track::Track;

pub fn read_track(file_path: &str) -> Track {
    // Attempt to read the file and handle potential errors
    let file = match lofty::read_from_path(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Access the first available tag (if any)
    let tag = match file.primary_tag() {
        Some(tag) => tag,
        None => {
            eprintln!("No tags found in the file.");
            std::process::exit(1);
        }
    };

    // Extract metadata from the tag
    let track =  Track {
        title: tag.title().unwrap_or(std::borrow::Cow::Borrowed("Unknown Title")).to_string(),
        artist: tag.artist().unwrap_or(std::borrow::Cow::Borrowed("Unknown Artist")).to_string(),
        album: tag.album().unwrap_or(std::borrow::Cow::Borrowed("Unknown Album")).to_string(),
        track_number: tag.track().map(|n| n as u32),
        duration_secs: file.properties().duration().as_secs() as u32, 
        path: std::path::PathBuf::from(file_path),
    }; 

    return track;
}  
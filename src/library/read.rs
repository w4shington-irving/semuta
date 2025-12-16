use lofty::{file::AudioFile, prelude::TaggedFileExt, tag::Accessor};
use crate::model::track::Track;
use std::error::Error;

pub fn read_track(file_path: &str) -> Result<Track, Box<dyn Error>> {
    // Attempt to read the file and handle potential errors
    let file = lofty::read_from_path(file_path)?;

    // Access the first available tag (if any)
    let tag = file.primary_tag().ok_or("No tags found in the file")?;

    // Extract metadata from the tag
    let track = Track {
        title: tag.title().unwrap_or(std::borrow::Cow::Borrowed("Unknown Title")).to_string(),
        artist: tag.artist().unwrap_or(std::borrow::Cow::Borrowed("Unknown Artist")).to_string(),
        album: tag.album().unwrap_or(std::borrow::Cow::Borrowed("Unknown Album")).to_string(),
        track_number: tag.track().map(|n| n as u32),
        duration_secs: file.properties().duration().as_secs() as u32,
        path: file_path.to_string(),
    };

    Ok(track)
}
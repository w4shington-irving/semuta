use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;
use crate::model::track::Track;
use crate::library::read::read_track;

/// Supported music file extensions
const SUPPORTED_EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a"];

/// Scans a directory for music files in parallel and returns a vector of `Track`
pub fn scan_files(dir: &str) -> Vec<Track> {
    WalkDir::new(dir)
        .into_iter()
        .par_bridge() // Parallelize the iterator with rayon
        .filter_map(|entry| {
            // Filter out invalid entries
            let entry = entry.ok()?;
            let path = entry.path();

            // Check if the file has a supported extension
            if path.is_file() && has_supported_extension(path) {
                // Attempt to read the track, log errors, and skip invalid files
                match read_track(path.to_str().unwrap()) {
                    Ok(track) => Some(track),
                    Err(e) => {
                        eprintln!("Error reading track {}: {}", path.display(), e);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect() // Collect the results into a Vec<Track>
}

/// Checks if a file has a supported music file extension
fn has_supported_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

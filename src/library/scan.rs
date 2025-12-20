use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;
use crate::db::append;
use crate::library::read::read_track as read;   


/// Supported music file extensions
/// Supported audio formats
const AUDIO_EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a"];

/// Scan all files in `root_dir`, read audio files and append tracks to the DB
pub fn scan_and_append(root_dir: &Path) {
    // Collect all audio files recursively
    let paths: Vec<_> = WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .map(|e| e.into_path())
        .collect();

    // Process files in parallel
    paths.par_iter().for_each(|path| {
        match read(path.to_str().unwrap()) {
            Ok(track) => {
                if let Err(e) = append(&track) {
                    eprintln!("Failed to append track {}: {}", path.display(), e);
                }
            }
            Err(e) => {
                eprintln!("Failed to read track {}: {}", path.display(), e);
            }
        }
    });
}
use crate::db;

pub mod read;
pub mod scan;

pub fn populate_library(dir: &str) {
    let tracks = scan::scan_files(dir);
    tracks.iter().for_each(|track| {
        db::append(track).expect("Failed to append track to database");
    });
}
use std::path::Path;
pub mod read;
pub mod scan;

pub fn populate_library(dir: &str) {
    scan::scan_and_append(Path::new(dir));
}
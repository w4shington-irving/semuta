use crate::library::populate_library;

mod model;
mod library;
mod db;
mod ui;
mod app;
mod audio;

fn main() {
    db::initialize_database().expect("Failed to initialize database");

    let music_dir = "/home/washington/Music";
    populate_library(music_dir);
    
    ui::start();
    
}
/*
TODO:
- Fix breaking tracks (eg. Pink Floyd - The Dark Side of the Moon)
 */
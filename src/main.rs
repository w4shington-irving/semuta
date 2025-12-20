use crate::library::populate_library;

mod model;
mod library;
mod db;
mod ui;
mod app;

fn main() {
    db::initialize_database().expect("Failed to initialize database");

    let music_dir = "/home/washington/Music";
    populate_library(music_dir);
    
    ui::start();
    
}
/*
TODO:
- Improve UX by storing previous selection indices when navigating views
- Add audio playback functionality */
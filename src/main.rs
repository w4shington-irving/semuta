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
    
    ui::display_library();
    
}
/*
TODO:
- Implement the UI rendering logic in `ui::display_ui()`
- Implement identifying structures by either name or id with enum variants */
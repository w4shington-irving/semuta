use crate::library::populate_library;
use crate::app::App;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
    

    let app = Arc::new(Mutex::new(App::new()));

    let app_clone = Arc::clone(&app);

    

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000)); // update every second
            let mut app = app_clone.lock().unwrap();
            app.update();
        }
    });

    ui::run(Arc::clone(&app)).expect("Failed to generate TUI");

    
    
}
/*
TODO:
- Make the UI more readable (make tracks bold)
- Support global shortcuts (eg. play/pause, next, previous)
- Fix breaking tracks (eg. Pink Floyd - The Dark Side of the Moon)
- Add playlists
- Add search
- Add shuffle
 */
use crate::{db::get_tracks_by_album_id, library::populate_library};

mod model;
mod library;
mod db;
mod ui; 


fn main() {
    db::initialize_database().expect("Failed to initialize database");

    let music_dir = "/home/washington/Music";
    populate_library(music_dir);


    if let Ok(tracks) = get_tracks_by_album_id(16) {
        for track in tracks {
            println!("{:?}", track);
        }
    }
    
    ui::display::print_library();
    
}
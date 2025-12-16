use crate::db;

pub fn print_library() {
    for artist in db::get_artists().expect("Failed to get artists") {
        println!("* {}", artist.name);
        for album in db::get_albums_by_artist_id(artist.id).expect("Failed to get albums") {
            println!("  - {}", album.title);
            for track in db::get_tracks_by_album_id(album.id).expect("Failed to get tracks") {
                println!("      [{}] - {} ({}m{}s)", 
                    track.track_number.unwrap_or(0), 
                    track.title, 
                    track.duration_secs / 60,
                    track.duration_secs % 60
                );
            }
            
        }
    }
}
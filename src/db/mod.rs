use crate::db::artists::{add_artist, artist_exists, get_artist_id};
use crate::db::albums::{add_album, album_exists, get_album_id};
use crate::model::track::Track;

pub mod schema;
pub mod tracks;
pub mod albums;
pub mod artists;

pub fn initialize_database() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("library.db")?;
    schema::create_library(&conn)
}

pub fn append(track: &Track) -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("library.db")?;

    // Check if the artist exists, and add if not
    if !artist_exists(&conn, &track.artist)? {
        add_artist(&conn, &track.artist)?;
    }

    // Get the artist ID
    let artist_id = get_artist_id(&conn, &track.artist)?.unwrap();

    // Check if the album exists, and add if not
    if !album_exists(&conn, &track.album, artist_id)? {
        add_album(&conn, &track.album, artist_id)?;
    }

    // Get the album ID
    let album_id = get_album_id(&conn, &track.album, artist_id)?.unwrap();
    // Add the track to the database
    tracks::add_track(&conn, track, album_id)
}
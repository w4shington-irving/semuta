use crate::db::artists::{add_artist, artist_exists, get_artist};
use crate::db::albums::{add_album, album_exists, get_album};
use crate::model::{track::Track, identifier::Identifier};

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
    if !artist_exists(&conn, Identifier::Name((&track.artist)))? {
        add_artist(&conn, &track.artist)?;
    } 

    let artist = get_artist(&conn, Identifier::Name((&track.artist)))?;
    // Get the artist ID
    

    // Check if the album exists, and add if not
    if !album_exists(&conn, Identifier::Name((&track.album)), artist.id)? {
        add_album(&conn, &track.album, artist.id)?;
    } 

    let album = get_album(&conn, Identifier::Name((&track.album)), artist.id)?;
    
    
    // Add the track to the database
    tracks::add_track(&conn, track, album.id)
}

pub fn get_artists() -> rusqlite::Result<Vec<crate::model::artist::Artist>> {
    let conn = rusqlite::Connection::open("library.db")?;
    artists::get_all_artists(&conn)
}

pub fn get_albums_by_artist_id(artist_id: i64) -> rusqlite::Result<Vec<crate::model::album::Album>> {
    let conn = rusqlite::Connection::open("library.db")?;
    albums::get_all_albums_by_artist_id(&conn, artist_id)
}
/* 
pub fn get_albums_by_artist_name(artist_name: &str) -> rusqlite::Result<Vec<crate::model::album::Album>> {
    let conn = rusqlite::Connection::open("library.db")?;
    let artist = artists::get_artist_by_name(&conn, artist_name)?;
    albums::get_all_albums_by_artist_id(&conn, artist.id)
}
*/
pub fn get_tracks_by_album_id(album_id: i64) -> rusqlite::Result<Vec<Track>> {
    let conn = rusqlite::Connection::open("library.db")?;
    tracks::get_tracks_by_album_id(&conn, album_id)
}
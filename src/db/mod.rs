use crate::db::artists::{add_artist, artist_exists, get_artist_internal};
use crate::db::albums::{add_album, album_exists};
use crate::model::{album, artist};
use crate::model::track::Track;
use crate::model::identifier::{ArtistIdentifier, AlbumIdentifier, TrackIdentifier};

pub mod schema;
pub mod tracks;
pub mod albums;
pub mod artists;



pub fn initialize_database() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("library.db")?;
    schema::create_library(&conn)
}

pub fn get_artist(artist_identifier: &ArtistIdentifier) -> rusqlite::Result<crate::model::artist::Artist> {
    let conn = rusqlite::Connection::open("library.db")?;
    artists::get_artist_internal(&conn, artist_identifier)
}

pub fn get_album(album_identifier: &AlbumIdentifier) -> rusqlite::Result<crate::model::album::Album> {
    let conn = rusqlite::Connection::open("library.db")?;
    albums::get_album_internal(&conn, album_identifier)
}

pub fn get_track(track_identifier: &TrackIdentifier) -> rusqlite::Result<crate::model::track::Track> {
    let conn = rusqlite::Connection::open("library.db")?;
    tracks::get_track_internal(&conn, track_identifier)
}



pub fn append(track: &Track) -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("library.db")?;
    
    if !artist_exists(&conn, ArtistIdentifier::Name(&track.artist_name))? {
        add_artist(&conn, &track.artist_name)?;
    }
    let artist = get_artist(&ArtistIdentifier::Name(&track.artist_name))?;

    if !album_exists(&conn, AlbumIdentifier::Name { name: &track.album_name, artist_id: artist.id })? {
        add_album(&conn, &track.album_name, artist.id)?;
    }
    let album = get_album(&AlbumIdentifier::Name { name: &track.album_name, artist_id: artist.id })?;


    // Add the track to the database
    tracks::add_track(&conn, track, album.id)?;
    
    Ok(())
}

pub fn get_artists() -> rusqlite::Result<Vec<crate::model::artist::Artist>> {
    let conn = rusqlite::Connection::open("library.db")?;
    artists::get_all_artists(&conn)
}


pub fn get_albums(artist_identifier: &ArtistIdentifier) -> rusqlite::Result<Vec<crate::model::album::Album>> {
    let conn = rusqlite::Connection::open("library.db")?;
    albums::get_albums_by_artist_internal(&conn, &artist_identifier)
}

pub fn get_tracks(album_identifier: &AlbumIdentifier) -> rusqlite::Result<Vec<Track>> {
    let conn = rusqlite::Connection::open("library.db")?;
    let mut tracks = tracks::get_tracks_by_album_internal(&conn, &album_identifier)?;
    tracks.sort_by_key(|t| t.track_number.unwrap_or(0));
    Ok(tracks)
}
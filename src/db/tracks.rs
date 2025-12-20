use crate::{db::get_artist, model::{album, identifier::{AlbumIdentifier, ArtistIdentifier, TrackIdentifier}, track::Track}};
use crate::db::get_album;
use rusqlite::Connection;

pub fn add_track(conn: &rusqlite::Connection, track: &Track, album_id: i64) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO tracks (album_id, title, track_number, duration_secs, path)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            album_id,
            track.title,
            track.track_number,
            track.duration_secs,
            track.path,
        ],
    )?;
    Ok(())
}

pub fn track_exists(conn: &Connection, id: TrackIdentifier) -> rusqlite::Result<bool> {
    match id {
        TrackIdentifier::Id(id) => {
            let mut stmt = conn.prepare("SELECT 1 FROM tracks WHERE id = ?1 LIMIT 1")?;
            Ok(stmt.exists([id])?)
        }
        TrackIdentifier::Name { name, album_id } => {
            let mut stmt = conn.prepare(
                "SELECT 1 FROM tracks WHERE title = ?1 AND album_id = ?2 LIMIT 1"
            )?;
            Ok(stmt.exists([name, &album_id.to_string()])?)
        }
    }
}


pub fn get_track_internal(conn: &rusqlite::Connection, identifier: &TrackIdentifier) -> rusqlite::Result<Track> {
    let mut stmt = conn.prepare("SELECT id, album_id, title, track_number, duration_secs, path FROM tracks WHERE id = ?1 OR (title = ?2 AND album_id = ?3)")?;
    match identifier {
        TrackIdentifier::Id(id) => {
            let mut rows = stmt.query(rusqlite::params![id, ""])?;
            if let Some(row) = rows.next()? {
                let album_identifier = AlbumIdentifier::Id(row.get(1)?);
                let album = get_album(&album_identifier)?;
                let artist_identifier = ArtistIdentifier::Id(album.artist_id);
                let artist = get_artist(&artist_identifier)?;
                return Ok(Track {
                    id: row.get(0)?,
                    album_id: row.get(1)?,
                    title: row.get(2)?,
                    track_number: row.get(3)?,
                    duration_secs: row.get(4)?,
                    path: row.get(5)?,
                    artist_name: artist.name,
                    album_name: album.title,
                    artist_id: artist.id,
                });
            }
            Err(rusqlite::Error::QueryReturnedNoRows)
        },
        TrackIdentifier::Name{name, album_id} => {
            let mut rows = stmt.query(rusqlite::params!["", name, album_id])?;
            if let Some(row) = rows.next()? {
                let album_identifier = AlbumIdentifier::Id(*album_id);
                let album = get_album(&album_identifier)?;
                let artist_identifier = ArtistIdentifier::Id(album.artist_id);
                let artist = get_artist(&artist_identifier)?;
                return Ok(Track {
                    id: row.get(0)?,
                    album_id: row.get(1)?,
                    title: row.get(2)?,
                    track_number: row.get(3)?,
                    duration_secs: row.get(4)?,
                    path: row.get(5)?,
                    artist_name: artist.name,
                    album_name: album.title,
                    artist_id: artist.id,
                });
            }
            Err(rusqlite::Error::QueryReturnedNoRows)
        },
    }
}

pub fn get_tracks_by_album_internal(conn: &rusqlite::Connection, album_identifier: &AlbumIdentifier) -> rusqlite::Result<Vec<Track>> {
    match album_identifier {
        AlbumIdentifier::Id(id) => {
            let mut stmt = conn.prepare("SELECT id, album_id, title, track_number, duration_secs, path FROM tracks WHERE album_id = ?1")?;
            let mut rows = stmt.query(rusqlite::params![id])?;
            let mut tracks = Vec::new();
            while let Some(row) = rows.next()? {
                let album = get_album(album_identifier).expect("Failed to read Album");
                let artist_identifier = ArtistIdentifier::Id(album.artist_id);
                let artist = get_artist(&artist_identifier)?;
                tracks.push(Track {
                    id: row.get(0)?,
                    album_id: row.get(1)?,
                    title: row.get(2)?,
                    track_number: row.get(3)?,
                    duration_secs: row.get(4)?, 
                    path: row.get(5)?,
                    artist_name: artist.name,
                    album_name: album.title,
                    artist_id: artist.id,
                });
            }
            Ok(tracks)
        },
        AlbumIdentifier::Name{name, artist_id} => {
            let mut stmt = conn.prepare("SELECT id, album_id, title, track_number, duration_secs, path FROM tracks WHERE album_id = (SELECT id FROM albums WHERE title = ?1 AND artist_id = ?2)")?;
            let mut rows = stmt.query(rusqlite::params![name, artist_id])?;
            let mut tracks = Vec::new();
            while let Some(row) = rows.next()? {
                let album = get_album(&album_identifier).expect("Failed to read Album");
                let artist_identifier = ArtistIdentifier::Id(album.artist_id);
                let artist = get_artist(&artist_identifier)?;
                tracks.push(Track {
                    id: row.get(0)?,
                    album_id: row.get(1)?,
                    title: row.get(2)?,
                    track_number: row.get(3)?,
                    duration_secs: row.get(4)?,
                    path: row.get(5)?,
                    artist_name: artist.name,
                    album_name: album.title,
                    artist_id: artist.id,
                });
            }
            Ok(tracks)
        },
    }
}
    
use crate::{db::{albums::get_album_by_id, artists::get_artist_by_id}, model::{track::Track}};

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

pub fn get_tracks_by_album_id(conn: &rusqlite::Connection, album_id: i64) -> rusqlite::Result<Vec<Track>> {
    let mut stmt = conn.prepare(
        "SELECT id, album_id, title, track_number, duration_secs, path
         FROM tracks
         WHERE album_id = ?1
         ORDER BY track_number ASC", // Sort tracks by track_number in ascending order
    )?;

    let mut rows = stmt.query(rusqlite::params![album_id])?;

    let mut tracks = Vec::new();
    while let Some(row) = rows.next()? {
        let track_title: String = row.get(2)?;
        let album_id: i64 = row.get(1)?;
        let album = get_album_by_id(conn, album_id)?;
        let artist = get_artist_by_id(conn, album.artist_id)?;
        tracks.push(Track {
            title: track_title,
            artist: artist.name,
            album: album.title,
            track_number: row.get(3)?,
            duration_secs: row.get(4)?,
            path: row.get(5)?,
            id: row.get(0)?,
        });
    }

    Ok(tracks)
}
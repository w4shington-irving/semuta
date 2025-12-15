use crate::model::track::Track;

pub fn add_track(conn: &rusqlite::Connection, track: &Track, album_id: i64) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO tracks (album_id, title, track_number, duration_secs, path) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            album_id,
            track.title,
            track.track_number,
            track.duration_secs,
            track.path.to_str().unwrap()
        ],
    )?;
    Ok(())
}
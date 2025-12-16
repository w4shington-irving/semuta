use crate::model::{album::Album};


pub fn add_album(conn: &rusqlite::Connection, title: &str, artist_id: i64) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO albums (title, artist_id) VALUES (?1, ?2)",
        rusqlite::params![title, artist_id],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn album_exists(conn: &rusqlite::Connection, title: &str, artist_id: i64) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM albums WHERE title = ?1 AND artist_id = ?2 LIMIT 1")?;
    let mut rows = stmt.query(rusqlite::params![title, artist_id])?;

    Ok(rows.next()?.is_some())
}

pub fn get_album_by_id(conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<Album> {
    let mut stmt = conn.prepare("SELECT id, title, artist_id FROM albums WHERE id = ?1")?;
    let mut rows = stmt.query(rusqlite::params![id])?;

    while let Some(row) = rows.next()? {
        return Ok(Album {
            id: row.get(0)?,
            artist_id: row.get(2)?,
            title: row.get(1)?
        });
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_album_by_name(conn: &rusqlite::Connection, title: &str, artist_id: i64) -> rusqlite::Result<Album> {
    let mut stmt = conn.prepare("SELECT id, title, artist_id FROM albums WHERE title = ?1 AND artist_id = ?2")?;
    let mut rows = stmt.query(rusqlite::params![title, artist_id])?;

    while let Some(row) = rows.next()? {
        return Ok(Album {
            id: row.get(0)?,
            artist_id: row.get(2)?,
            title: row.get(1)?,
        });
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}   

pub fn get_all_albums_by_artist_id(conn: &rusqlite::Connection, artist_id: i64) -> rusqlite::Result<Vec<Album>> {
    let mut stmt = conn.prepare("SELECT id, title, artist_id FROM albums WHERE artist_id = ?1")?;
    let mut rows = stmt.query(rusqlite::params![artist_id])?;

    let mut albums = Vec::new();
    while let Some(row) = rows.next()? {
        albums.push(Album {
            id: row.get(0)?,
            artist_id: row.get(2)?,
            title: row.get(1)?,
        });
    }

    Ok(albums) 
}

/* 
pub fn get_artist_by_album_id(conn: &rusqlite::Connection, album_id: i64) -> rusqlite::Result<Artist> {
    let mut stmt = conn.prepare("SELECT artist_id FROM albums WHERE id = ?1")?;
    let mut rows = stmt.query(rusqlite::params![album_id])?;

    if let Some(row) = rows.next()? {
        let artist_id: i64 = row.get(0)?;
        Ok(get_artist_by_id(conn, artist_id)?)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}
*/
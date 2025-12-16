use crate::model::artist::Artist;

pub fn add_artist(conn: &rusqlite::Connection, name: &str) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO artists (name) VALUES (?1)",
        rusqlite::params![name],
    )?;

    
    Ok(conn.last_insert_rowid())
}

pub fn get_artist_by_id(conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<Artist> {
    let mut stmt = conn.prepare("SELECT id, name FROM artists WHERE id = ?1")?;
    let mut rows = stmt.query(rusqlite::params![id])?;

    while let Some(row) = rows.next()? {
        return Ok(Artist {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_artist_by_name(conn: &rusqlite::Connection, name: &str) -> rusqlite::Result<Artist> {
    let mut stmt = conn.prepare("SELECT id, name FROM artists WHERE name = ?1")?;
    let mut rows = stmt.query(rusqlite::params![name])?;

    while let Some(row) = rows.next()? {
        return Ok(Artist {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn artist_exists(conn: &rusqlite::Connection, name: &str) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM artists WHERE name = ?1 LIMIT 1")?;
    let mut rows = stmt.query(rusqlite::params![name])?;

    Ok(rows.next()?.is_some())
}

pub fn get_all_artists(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<Artist>> {
    let mut stmt = conn.prepare("SELECT id, name FROM artists")?;
    let mut rows = stmt.query([])?;

    let mut artists = Vec::new();
    while let Some(row) = rows.next()? {
        artists.push(Artist {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }

    Ok(artists)
}
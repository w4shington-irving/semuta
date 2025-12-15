pub fn add_album(conn: &rusqlite::Connection, title: &str, artist_id: i64) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO albums (title, artist_id) VALUES (?1, ?2)",
        rusqlite::params![title, artist_id],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_album_id(conn: &rusqlite::Connection, title: &str, artist_id: i64) -> rusqlite::Result<Option<i64>> {
    let mut stmt = conn.prepare("SELECT id FROM albums WHERE title = ?1 AND artist_id = ?2")?;
    let mut rows = stmt.query(rusqlite::params![title, artist_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub fn album_exists(conn: &rusqlite::Connection, title: &str, artist_id: i64) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM albums WHERE title = ?1 AND artist_id = ?2 LIMIT 1")?;
    let mut rows = stmt.query(rusqlite::params![title, artist_id])?;

    Ok(rows.next()?.is_some())
}
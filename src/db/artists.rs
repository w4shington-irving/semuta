pub fn add_artist(conn: &rusqlite::Connection, name: &str) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO artists (name) VALUES (?1)",
        rusqlite::params![name],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_artist_id(conn: &rusqlite::Connection, name: &str) -> rusqlite::Result<Option<i64>> {
    let mut stmt = conn.prepare("SELECT id FROM artists WHERE name = ?1")?;
    let mut rows = stmt.query(rusqlite::params![name])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub fn artist_exists(conn: &rusqlite::Connection, name: &str) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM artists WHERE name = ?1 LIMIT 1")?;
    let mut rows = stmt.query(rusqlite::params![name])?;

    Ok(rows.next()?.is_some())
}
use rusqlite::{Connection, Result};

// Create the tracks table if it doesn't exist
pub fn create_library(conn: &Connection) -> Result<()> {
    // Create the artists table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS artists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // Create the albums table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albums (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            artist_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            UNIQUE(artist_id, title),
            FOREIGN KEY (artist_id) REFERENCES artists(id)
        )",
        [],
    )?;

    // Create the tracks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            album_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            track_number INTEGER,
            duration_secs INTEGER NOT NULL,
            path TEXT NOT NULL UNIQUE,
            UNIQUE(album_id, title),
            FOREIGN KEY (album_id) REFERENCES albums(id)
        )",
        [],
    )?;

    Ok(())
}

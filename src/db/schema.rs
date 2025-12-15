use rusqlite::{Connection, Result};

// Create the tracks table if it doesn't exist
pub fn create_library(conn: &Connection) -> Result<()> {
    // Create the artists table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS artists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
        [],
    )?;

    // Create the albums table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albums (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            artist_id INTEGER NOT NULL,
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
            FOREIGN KEY (album_id) REFERENCES albums(id)
        )",
        [],
    )?;

    Ok(())
}

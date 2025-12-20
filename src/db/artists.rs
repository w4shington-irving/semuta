use crate::model::artist::Artist;
use crate::db::Identifier;

pub fn add_artist(conn: &rusqlite::Connection, name: &str) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO artists (name) VALUES (?1)",
        rusqlite::params![name],
    )?;

    
    Ok(conn.last_insert_rowid())
}

pub fn get_artist(conn: &rusqlite::Connection, identifier: Identifier) -> rusqlite::Result<Artist> {
    let mut stmt = conn.prepare("SELECT id, name FROM artists WHERE id = ?1 OR name = ?2")?;
    match identifier {
        Identifier::Id(id) => {
            let mut rows = stmt.query(rusqlite::params![id, ""])?;
            if let Some(row) = rows.next()? {
                return Ok(Artist {
                    id: row.get(0)?,
                    name: row.get(1)?,
                });
            }
            Err(rusqlite::Error::QueryReturnedNoRows)
        },
        Identifier::Name(name) => {
            let mut rows = stmt.query(rusqlite::params!["", name])?;
            if let Some(row) = rows.next()? {
                return Ok(Artist {
                    id: row.get(0)?,
                    name: row.get(1)?,
                });
            }
            Err(rusqlite::Error::QueryReturnedNoRows)
        },
    }
    
}

pub fn artist_exists(conn: &rusqlite::Connection, identifier: Identifier) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM artists WHERE name = ?1 LIMIT 1")?;
    match identifier {
        Identifier::Id(id) => {
            let exists: i32 = stmt.query_row(rusqlite::params![id], |row| row.get(0))?;
            Ok(exists != 0)
        },
        Identifier::Name(name) => {
            let exists: i32 = stmt.query_row(rusqlite::params![name], |row| row.get(0))?;
            Ok(exists != 0)
        },
    }
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
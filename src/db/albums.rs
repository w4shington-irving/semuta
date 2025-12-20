use rusqlite::{Connection};
use crate::model::{album::Album, identifier::AlbumIdentifier, identifier::ArtistIdentifier};


pub fn add_album(conn: &rusqlite::Connection, title: &str, artist_id: i64) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO albums (title, artist_id) VALUES (?1, ?2)",
        rusqlite::params![title, artist_id],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn album_exists(conn: &Connection, id: AlbumIdentifier) -> rusqlite::Result<bool> {
    match id {
        AlbumIdentifier::Id(id) => {
            let mut stmt = conn.prepare("SELECT 1 FROM albums WHERE id = ?1 LIMIT 1")?;
            Ok(stmt.exists([id])?)
        }
        AlbumIdentifier::Name { name, artist_id } => {
            let mut stmt = conn.prepare(
                "SELECT 1 FROM albums WHERE title = ?1 AND artist_id = ?2 LIMIT 1"
            )?;
            Ok(stmt.exists([name, &artist_id.to_string()])?)
        }
    }
}




pub fn get_album_internal(conn: &Connection, identifier: &AlbumIdentifier) -> rusqlite::Result<Album> {
    
    
    match identifier {
        AlbumIdentifier::Id(id) => {
            let mut stmt = conn.prepare("SELECT id, artist_id, title FROM albums WHERE id = ?1")?;
            let mut rows = stmt.query(rusqlite::params![id])?;
            while let Some(row) = rows.next()? {
                return Ok(Album {
                    id: row.get(0)?,
                    artist_id: row.get(1)?,
                    title: row.get(2)?
                });
            }
        },
        AlbumIdentifier::Name{name, artist_id} => {
            let mut stmt = conn.prepare("SELECT id, artist_id, title FROM albums WHERE title = ?1 AND artist_id = ?2")?;
            let mut rows = stmt.query(rusqlite::params![name, artist_id])?;
            while let Some(row) = rows.next()? {
                return Ok(Album {
                    id: row.get(0)?,
                    artist_id: row.get(1)?,
                    title: row.get(2)?
                });
            }
        },
    
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}   

pub fn get_albums_by_artist_internal(conn: &rusqlite::Connection, artist_identifier: &ArtistIdentifier) -> rusqlite::Result<Vec<Album>> {
    match artist_identifier {
        ArtistIdentifier::Id(id) => {
            let mut stmt = conn.prepare("SELECT id, title, artist_id FROM albums WHERE artist_id = ?1")?;
            let mut rows = stmt.query(rusqlite::params![id])?;
            let mut albums = Vec::new();
            while let Some(row) = rows.next()? {
                albums.push(Album {
                    id: row.get(0)?,
                    artist_id: row.get(2)?,
                    title: row.get(1)?,
                });
            }
            Ok(albums) 
        },
        ArtistIdentifier::Name(name) => {
            let mut stmt = conn.prepare("SELECT id, title, artist_id FROM albums WHERE artist_id = (SELECT id FROM artists WHERE name = ?1)")?;
            let mut rows = stmt.query(rusqlite::params![name])?;
            let mut albums = Vec::new();
            while let Some(row) = rows.next()? {
                albums.push(Album {
                    id: row.get(0)?,
                    artist_id: row.get(2)?,
                    title: row.get(1)?,
                });
            }
            Ok(albums) 
        },
    }
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
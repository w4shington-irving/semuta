pub mod schema;

pub fn initialize_database() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("library.db")?;
    schema::create_library(&conn)
}
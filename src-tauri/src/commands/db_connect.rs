extern crate rusqlite;
use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                  id              INTEGER PRIMARY KEY,
                  description     TEXT NOT NULL,
                  completed       INTEGER NOT NULL
                  )",
        [],
    )?;
    Ok(conn)
}

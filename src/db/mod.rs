pub mod note_repository;
use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_db<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            folder TEXT
        )",
        [],
    )?;

    // Migration: Add folder column if it doesn't exist
    let _ = conn.execute("ALTER TABLE notes ADD COLUMN folder TEXT", []);

    // Create folders table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders (
            name TEXT PRIMARY KEY
        )",
        [],
    )?;

    // Insert default folders if table is empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM folders", [], |row| row.get(0))?;
    if count == 0 {
        conn.execute("INSERT INTO folders (name) VALUES ('Personal')", [])?;
        conn.execute("INSERT INTO folders (name) VALUES ('Work')", [])?;
    }

    Ok(conn)
}

pub fn get_memory_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

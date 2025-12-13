use crate::models::note::Note;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

pub struct NoteRepository {
    conn: Connection,
}

impl NoteRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, note: &Note) -> Result<()> {
        self.conn.execute(
            "INSERT INTO notes (id, title, content, created_at, updated_at, folder) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                note.id.to_string(),
                note.title,
                note.content,
                note.created_at.to_rfc3339(),
                note.updated_at.to_rfc3339(),
                note.folder,
            ],
        )?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare("SELECT id, title, content, created_at, updated_at, folder FROM notes ORDER BY updated_at DESC")?;
        let note_iter = stmt.query_map([], |row| {
            Ok(Note {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap_or(Utc::now().into())
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or(Utc::now().into())
                    .with_timezone(&Utc),
                folder: row.get(5)?,
            })
        })?;

        let mut notes = Vec::new();
        for note in note_iter {
            notes.push(note?);
        }
        Ok(notes)
    }

    pub fn update(&self, note: &Note) -> Result<()> {
        self.conn.execute(
            "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3, folder = ?4 WHERE id = ?5",
            params![
                note.title,
                note.content,
                note.updated_at.to_rfc3339(),
                note.folder,
                note.id.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: Uuid) -> Result<()> {
        self.conn
            .execute("DELETE FROM notes WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    pub fn rename_folder(&self, old_name: &str, new_name: &str) -> Result<()> {
        // Update the folder name in the folders table
        self.conn.execute(
            "UPDATE folders SET name = ?1 WHERE name = ?2",
            params![new_name, old_name],
        )?;

        // Update the folder column in the notes table
        self.conn.execute(
            "UPDATE notes SET folder = ?1 WHERE folder = ?2",
            params![new_name, old_name],
        )?;
        Ok(())
    }

    pub fn get_folders(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM folders ORDER BY name")?;
        let folder_iter = stmt.query_map([], |row| row.get(0))?;

        let mut folders = Vec::new();
        for folder in folder_iter {
            folders.push(folder?);
        }
        Ok(folders)
    }

    pub fn add_folder(&self, name: &str) -> Result<()> {
        self.conn
            .execute("INSERT INTO folders (name) VALUES (?1)", params![name])?;
        Ok(())
    }
}

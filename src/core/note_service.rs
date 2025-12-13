use crate::db::note_repository::NoteRepository;
use crate::models::note::Note;
use rusqlite::Result;
use uuid::Uuid;

pub struct NoteService {
    repo: NoteRepository,
}

impl NoteService {
    pub fn new(repo: NoteRepository) -> Self {
        Self { repo }
    }

    pub fn get_all_notes(&self) -> Result<Vec<Note>> {
        self.repo.get_all()
    }

    pub fn create_note(&self, title: String, content: String) -> Result<Note> {
        let note = Note::new(title, content);
        self.repo.create(&note)?;
        Ok(note)
    }

    pub fn update_note(&self, note: &Note) -> Result<()> {
        self.repo.update(note)
    }

    pub fn delete_note(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id)
    }

    pub fn rename_folder(&self, old_name: &str, new_name: &str) -> Result<()> {
        self.repo.rename_folder(old_name, new_name)
    }

    pub fn get_folders(&self) -> Result<Vec<String>> {
        self.repo.get_folders()
    }

    pub fn add_folder(&self, name: &str) -> Result<()> {
        self.repo.add_folder(name)
    }
}

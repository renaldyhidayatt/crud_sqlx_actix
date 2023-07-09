use std::sync::Arc;

use sqlx::Error;
use uuid::Uuid;

use crate::{models::NoteModel, repository::NoteRepository};

#[derive(Clone)]
pub struct NoteService {
    repository: Arc<NoteRepository>,
}

impl NoteService {
    pub fn new(repository: Arc<NoteRepository>) -> Self {
        NoteService { repository }
    }

    pub async fn get_notes(&self) -> Result<Vec<NoteModel>, Error> {
        self.repository.get_notes().await
    }

    pub async fn get_note_id(&self, id: Uuid) -> Result<Option<NoteModel>, Error> {
        self.repository.get_note_id(id).await
    }

    pub async fn create_note(&self, title: &str, content: &str) -> Result<NoteModel, Error> {
        self.repository.create_note(title, content).await
    }

    pub async fn update_note(
        &self,
        id: Uuid,
        title: &str,
        content: &str,
    ) -> Result<Option<NoteModel>, Error> {
        self.repository.update_note(id, title, content).await
    }

    pub async fn delete_note(&self, id: Uuid) -> Result<(), Error> {
        self.repository.delete(id).await
    }
}

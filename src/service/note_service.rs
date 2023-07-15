use async_trait::async_trait;
use std::sync::Arc;

use sqlx::Error;
use uuid::Uuid;

use crate::{
    abstract_trait::{DynNoteRepository, NoteServiceTrait},
    response::NoteResponse,
};

#[derive(Clone)]
pub struct NoteService {
    repository: DynNoteRepository,
}

impl NoteService {
    pub fn new(repository: DynNoteRepository) -> Self {
        Self { repository }
    }
}
#[async_trait]
impl NoteServiceTrait for NoteService {
    async fn get_notes(&self) -> anyhow::Result<Vec<NoteResponse>> {
        let notes = self.repository.get_notes().await?;
        let note_responses: Vec<NoteResponse> = notes.into_iter().map(|note| note.into()).collect();
        Ok(note_responses)
    }

    async fn get_note_id(&self, id: Uuid) -> anyhow::Result<Option<NoteResponse>> {
        let note = self.repository.get_note_id(id).await?;
        match note {
            Some(note) => Ok(Some(note.into())),
            None => Ok(None),
        }
    }

    async fn create_note(&self, title: &str, content: &str) -> anyhow::Result<NoteResponse> {
        let note = self.repository.create_note(title, content).await?;
        Ok(note.into())
    }

    async fn update_note(
        &self,
        id: Uuid,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Option<NoteResponse>> {
        let note = self.repository.update_note(id, title, content).await?;
        match note {
            Some(note) => Ok(Some(note.into())),
            None => Ok(None),
        }
    }

    async fn delete_note(&self, id: Uuid) -> anyhow::Result<()> {
        self.repository.delete(id).await?;
        Ok(())
    }
}

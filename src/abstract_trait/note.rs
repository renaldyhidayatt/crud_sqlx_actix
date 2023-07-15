use std::sync::Arc;

use async_trait::async_trait;

use crate::{models::NoteModel, response::NoteResponse};

use sqlx::Error;
use uuid::Uuid;

pub type DynNoteRepository = Arc<dyn NoteRepositoryTrait + Send + Sync>;
pub type DynNoteService = Arc<dyn NoteServiceTrait + Send + Sync>;

#[async_trait]
pub trait NoteRepositoryTrait {
    async fn get_notes(&self) -> Result<Vec<NoteModel>, Error>;
    async fn get_note_id(&self, id: Uuid) -> Result<Option<NoteModel>, Error>;
    async fn create_note(&self, title: &str, content: &str) -> Result<NoteModel, Error>;
    async fn update_note(
        &self,
        id: Uuid,
        title: &str,
        content: &str,
    ) -> Result<Option<NoteModel>, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[async_trait]
pub trait NoteServiceTrait {
    async fn get_notes(&self) -> anyhow::Result<Vec<NoteResponse>>;
    async fn get_note_id(&self, id: Uuid) -> anyhow::Result<Option<NoteResponse>>;
    async fn create_note(&self, title: &str, content: &str) -> anyhow::Result<NoteResponse>;
    async fn update_note(
        &self,
        id: Uuid,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Option<NoteResponse>>;
    async fn delete_note(&self, id: Uuid) -> anyhow::Result<()>;
}

use crate::config::ConnectionPool;
use chrono::Utc;
use sqlx::Error;
use uuid::Uuid;

use crate::models::NoteModel;

pub struct NoteRepository {
    pub db_pool: ConnectionPool,
}

impl NoteRepository {
    pub async fn get_notes(&self) -> Result<Vec<NoteModel>, Error> {
        let notes = sqlx::query_as::<_, NoteModel>("SELECT * FROM notes")
            .fetch_all(&self.db_pool)
            .await?;

        Ok(notes)
    }

    pub async fn get_note_id(&self, id: Uuid) -> Result<Option<NoteModel>, Error> {
        let todo = sqlx::query_as::<_, NoteModel>("SELECT * FROM notes WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(todo)
    }

    pub async fn create_note(&self, title: &str, content: &str) -> Result<NoteModel, Error> {
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let note = sqlx::query_as::<_, NoteModel>(
            "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(title)
        .bind(content)
        .bind(created_at)
        .bind(updated_at)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(note)
    }

    pub async fn update_note(
        &self,
        id: Uuid,
        title: &str,
        content: &str,
    ) -> Result<Option<NoteModel>, Error> {
        let updated_at = Utc::now();

        let note = sqlx::query_as::<_, NoteModel>(
            "UPDATE notes SET title = $1, content = $2,  updated_at = $3 WHERE id = $4 RETURNING *",
        )
        .bind(title)
        .bind(content)
        .bind(updated_at)
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(note)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM notes
            WHERE id = $1
            "#,
            id,
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}

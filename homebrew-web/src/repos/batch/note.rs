use crate::error::ApiError;
use crate::repos::batch::BatchId;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBatchNote {
    pub batch_id: i32,
    pub target: String,
    pub time: chrono::NaiveDateTime,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchNoteId {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchNoteUpdate {
    pub id: i32,
    pub batch_id: i32,
    pub target: String,
    pub time: chrono::NaiveDateTime,
    pub value: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchNote {
    pub id: i32,
    pub batch_id: i32,
    pub target: String,
    pub time: String,
    pub value: String,
}

impl BatchNote {
    pub async fn insert(db: &PgPool, input: NewBatchNote) -> Result<Self, ApiError> {
        let trans = db.begin().await?;

        let beer = sqlx::query_as!(
            DbBatchNote,
            r#"
            INSERT INTO batch_note (batch_id, target, time, value) VALUES ($1, $2, $3, $4)
            RETURNING id, batch_id, target, time, value
            "#,
            input.batch_id,
            input.target,
            input.time,
            input.value,
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(beer.into())
    }

    pub async fn delete(db: &PgPool, input: BatchNoteId) -> Result<(), ApiError> {
        let trans = db.begin().await?;

        sqlx::query!("DELETE FROM batch_note WHERE id = $1", input.id)
            .execute(db)
            .await?;

        trans.commit().await?;
        Ok(())
    }

    pub async fn list(db: &PgPool, input: BatchId) -> Result<Vec<Self>, ApiError> {
        let db_ingredients = sqlx::query_as!(
            DbBatchNote,
            r#"
            SELECT id, batch_id, target, time, value FROM batch_note
            WHERE batch_id = $1
            ORDER BY id
            "#,
            input.batch_id,
        )
        .fetch_all(db)
        .await?;

        let ingredients: Vec<BatchNote> =
            db_ingredients.into_iter().map(|row| row.into()).collect();
        Ok(ingredients.into())
    }

    pub async fn update(db: &PgPool, input: BatchNoteUpdate) -> Result<Self, ApiError> {
        let ingredient = sqlx::query_as!(
            DbBatchNote,
            r#"
            UPDATE batch_note
            SET time=$4, value=$5
            WHERE id = $1 AND batch_id = $2 AND target = $3
            RETURNING id, batch_id, target, time, value
            "#,
            input.id,
            input.batch_id,
            input.target,
            input.time,
            input.value,
        )
        .fetch_one(db)
        .await?;

        Ok(ingredient.into())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct DbBatchNote {
    pub id: i32,
    pub batch_id: i32,
    pub target: String,
    pub time: chrono::NaiveDateTime,
    pub value: String,
}

impl From<DbBatchNote> for BatchNote {
    fn from(m: DbBatchNote) -> Self {
        Self {
            id: m.id,
            batch_id: m.batch_id,
            target: m.target,
            time: m.time.to_string(),
            value: m.value,
        }
    }
}

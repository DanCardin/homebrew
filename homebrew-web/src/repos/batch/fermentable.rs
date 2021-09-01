use crate::error::ApiError;
use crate::repos::batch::BatchId;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;
use std::convert::TryFrom;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBatchFermentable {
    pub batch_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchFermentableId {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchFermentableUpdate {
    pub id: i32,
    pub fermentable_id: Option<i32>,
    pub amount: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchFermentable {
    pub id: i32,
    pub batch_id: i32,
    pub fermentable_id: Option<i32>,
    pub amount: f64,
    pub time: Option<String>,
    pub unit: Option<String>,
}

impl BatchFermentable {
    pub async fn insert(db: &PgPool, input: NewBatchFermentable) -> Result<Self, ApiError> {
        let trans = db.begin().await?;

        let beer = sqlx::query_as!(
            DbBatchFermentable,
            r#"
            INSERT INTO batch_fermentable (batch_id) VALUES ($1)
            RETURNING id, batch_id, fermentable_id, amount, time, unit
            "#,
            input.batch_id,
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(beer.into())
    }

    pub async fn delete(db: &PgPool, input: BatchFermentableId) -> Result<(), ApiError> {
        let trans = db.begin().await?;

        sqlx::query!("DELETE FROM batch_fermentable WHERE id = $1", input.id,)
            .execute(db)
            .await?;

        trans.commit().await?;
        Ok(())
    }

    pub async fn list(db: &PgPool, input: BatchId) -> Result<Vec<Self>, ApiError> {
        let db_ingredients = sqlx::query_as!(
            DbBatchFermentable,
            r#"
            SELECT id, batch_id, fermentable_id, amount, time, unit FROM batch_fermentable
            WHERE batch_id = $1
            ORDER BY id
            "#,
            input.batch_id,
        )
        .fetch_all(db)
        .await?;

        let ingredients: Vec<BatchFermentable> =
            db_ingredients.into_iter().map(|row| row.into()).collect();
        Ok(ingredients.into())
    }

    pub async fn update(db: &PgPool, input: BatchFermentableUpdate) -> Result<Self, ApiError> {
        let ingredient = sqlx::query_as!(
            DbBatchFermentable,
            r#"
            UPDATE batch_fermentable
            SET fermentable_id=$2, amount=$3, unit=$4
            WHERE id = $1
            RETURNING id, batch_id, fermentable_id, amount, time, unit
            "#,
            input.id,
            input.fermentable_id,
            sqlx::types::BigDecimal::try_from(input.amount).unwrap(),
            input.unit,
        )
        .fetch_one(db)
        .await?;

        Ok(ingredient.into())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct DbBatchFermentable {
    pub id: i32,
    pub batch_id: i32,
    pub fermentable_id: Option<i32>,
    pub amount: sqlx::types::BigDecimal,
    pub time: Option<chrono::NaiveTime>,
    pub unit: Option<String>,
}

impl From<DbBatchFermentable> for BatchFermentable {
    fn from(m: DbBatchFermentable) -> Self {
        Self {
            id: m.id,
            batch_id: m.batch_id,
            fermentable_id: m.fermentable_id,
            amount: m.amount.to_f64().unwrap_or(0.),
            time: m.time.map(|t| t.to_string()),
            unit: m.unit,
        }
    }
}

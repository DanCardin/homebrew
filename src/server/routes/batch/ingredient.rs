use crate::error::ApiError;
use actix_web::web::{Data, Json};
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;
use tracing;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    Lb,
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientId {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchId {
    pub batch_id: i32,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewIngredient {
    pub batch_id: i32,
    pub kind: String,
    pub unit: Units,
}

#[derive(Debug)]
pub struct DbIngredient {
    pub id: i32,
    pub batch_id: i32,
    pub kind: String,
    pub unit: String,
    pub name: String,
    pub amount: sqlx::types::BigDecimal,
    pub time: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Ingredient {
    pub id: i32,
    pub batch_id: i32,
    pub kind: String,
    pub unit: Units,
    pub name: String,
    pub amount: f64,
    pub time: Option<String>,
}

impl From<DbIngredient> for Ingredient {
    fn from(m: DbIngredient) -> Self {
        Self {
            id: m.id,
            batch_id: m.batch_id,
            kind: m.kind,
            unit: serde_json::from_str(&m.unit).unwrap_or(Units::None),
            name: m.name,
            amount: m.amount.to_f64().unwrap_or(0.0),
            time: m.time,
        }
    }
}

impl Ingredient {
    pub async fn insert(db: &PgPool, new_ingredient: NewIngredient) -> Result<Self, ApiError> {
        let trans = db.begin().await?;

        let beer = sqlx::query_as!(
            DbIngredient,
            r#"
            INSERT INTO batch_ingredient (batch_id, kind, unit) VALUES ($1, $2, $3)
            RETURNING id, batch_id, kind, unit, name, amount, time
            "#,
            new_ingredient.batch_id,
            new_ingredient.kind,
            serde_json::to_string(&new_ingredient.unit).unwrap_or("".to_string()),
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(beer.into())
    }

    pub async fn delete(db: &PgPool, ingredient_id: i32) -> Result<(), ApiError> {
        let trans = db.begin().await?;

        sqlx::query_as!(
            DbIngredient,
            "DELETE FROM batch_ingredient WHERE id = $1",
            ingredient_id,
        )
        .execute(db)
        .await?;

        trans.commit().await?;
        Ok(())
    }

    pub async fn list(db: &PgPool, batch_id: i32, kind: String) -> Result<Vec<Self>, ApiError> {
        let db_ingredients = sqlx::query_as!(
            DbIngredient,
            r#"
            SELECT id, batch_id, kind, unit, name, amount, time FROM batch_ingredient
            WHERE batch_id = $1 and kind = $2
            "#,
            batch_id,
            kind,
        )
        .fetch_all(db)
        .await?;

        let ingredients: Vec<Ingredient> =
            db_ingredients.into_iter().map(|row| row.into()).collect();
        Ok(ingredients.into())
    }
}

#[tracing::instrument(
    skip(db, new_ingredient),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn new(
    db: Data<PgPool>,
    new_ingredient: Json<NewIngredient>,
) -> Result<Json<Ingredient>, ApiError> {
    let ingredient = Ingredient::insert(db.get_ref(), new_ingredient.0).await?;
    info!("{:?}", ingredient);
    Ok(Json(ingredient))
}

#[tracing::instrument(
    skip(db, batch_id),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn list(
    db: Data<PgPool>,
    batch_id: Json<BatchId>,
) -> Result<Json<Vec<Ingredient>>, ApiError> {
    let ingredients = Ingredient::list(db.get_ref(), batch_id.0.batch_id, batch_id.0.kind).await?;
    info!("{:?}", ingredients);
    Ok(Json(ingredients))
}

#[tracing::instrument(
    skip(db, ingredient_id),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn delete(
    db: Data<PgPool>,
    ingredient_id: Json<IngredientId>,
) -> Result<Json<()>, ApiError> {
    Ingredient::delete(db.get_ref(), ingredient_id.0.id).await?;
    Ok(Json(()))
}

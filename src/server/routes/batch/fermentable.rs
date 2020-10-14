use crate::error::ApiError;
use crate::repos::fermentable::FermentableId;
use actix_web::web::{Data, Json};
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;
use tracing;
use tracing::info;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBatchFermentable {
    pub batch_id: i32,
    pub kind: String,
    pub unit: Units,
}

#[derive(Debug, Serialize)]
pub struct BatchFermentable {
    pub id: i32,
    pub batch_id: i32,
    pub kind: String,
    pub unit: Units,
    pub name: String,
    pub amount: f64,
    pub time: Option<String>,
}

impl BatchFermentable {
    pub async fn insert(db: &PgPool, new_ingredient: NewIngredient) -> Result<Self, ApiError> {
        todo!();
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
        todo!();
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
        todo!();
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

#[tracing::instrument(skip(db, new_ingredient))]
pub async fn new(
    db: Data<PgPool>,
    new_ingredient: Json<NewIngredient>,
) -> Result<Json<Ingredient>, ApiError> {
    todo!();
    let ingredient = Ingredient::insert(db.get_ref(), new_ingredient.0).await?;
    info!("{:?}", ingredient);
    Ok(Json(ingredient))
}

#[tracing::instrument(skip(db, batch_id))]
pub async fn list(
    db: Data<PgPool>,
    batch_id: Json<BatchId>,
) -> Result<Json<Vec<Ingredient>>, ApiError> {
    todo!();
    let ingredients = Ingredient::list(db.get_ref(), batch_id.0.batch_id, batch_id.0.kind).await?;
    info!("{:?}", ingredients);
    Ok(Json(ingredients))
}

#[tracing::instrument(skip(db, ingredient_id))]
pub async fn delete(
    db: Data<PgPool>,
    ingredient_id: Json<IngredientId>,
) -> Result<Json<()>, ApiError> {
    todo!();
    Ingredient::delete(db.get_ref(), ingredient_id.0.id).await?;
    Ok(Json(()))
}

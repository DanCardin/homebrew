use crate::error::ApiError;
use crate::repos::batch::fermentable::{
    BatchFermentable, BatchFermentableId, BatchFermentableUpdate, NewBatchFermentable,
};
use crate::routes::batch::BatchId;
use actix_web::web::{Data, Json};
use sqlx::postgres::PgPool;
use tracing;
use tracing::info;

#[tracing::instrument(skip(db))]
pub async fn new(
    db: Data<PgPool>,
    new_batch_fermentable: Json<NewBatchFermentable>,
) -> Result<Json<BatchFermentable>, ApiError> {
    let result = BatchFermentable::insert(db.get_ref(), new_batch_fermentable.0).await?;
    info!("{:?}", result);
    Ok(Json(result))
}

#[tracing::instrument(skip(db))]
pub async fn list(
    db: Data<PgPool>,
    batch_id: Json<BatchId>,
) -> Result<Json<Vec<BatchFermentable>>, ApiError> {
    let results = BatchFermentable::list(db.get_ref(), batch_id.0).await?;
    info!("{:?}", results);
    Ok(Json(results))
}

#[tracing::instrument(skip(db))]
pub async fn update(
    db: Data<PgPool>,
    batch_update: Json<BatchFermentableUpdate>,
) -> Result<Json<BatchFermentable>, ApiError> {
    let result = BatchFermentable::update(db.get_ref(), batch_update.0).await?;
    info!("{:?}", result);
    Ok(Json(result))
}

#[tracing::instrument(skip(db))]
pub async fn delete(
    db: Data<PgPool>,
    batch_fermentable_id: Json<BatchFermentableId>,
) -> Result<Json<()>, ApiError> {
    BatchFermentable::delete(db.get_ref(), batch_fermentable_id.0).await?;
    Ok(Json(()))
}

use crate::error::ApiError;
use crate::repos::fermentable::{Fermentable, FermentableId, FermentableSearch, NewFermentable};
use actix_multipart::Multipart;
use actix_web::error::BlockingError;
use actix_web::web;
use actix_web::web::{Data, Json};
use bytes::buf::BufExt;
use futures::TryStreamExt;
use sqlx;
use sqlx::postgres::PgPool;
use tracing;

#[tracing::instrument(skip(db))]
pub async fn new(
    db: Data<PgPool>,
    new_fermentable: Json<NewFermentable>,
) -> Result<Json<Fermentable>, ApiError> {
    let fermentable = Fermentable::insert(db.get_ref(), new_fermentable.0).await?;
    Ok(Json(fermentable))
}

#[tracing::instrument(skip(db, payload))]
pub async fn import(db: Data<PgPool>, mut payload: Multipart) -> Result<Json<()>, ApiError> {
    let field = match payload.try_next().await {
        Ok(Some(field)) => field,
        _ => {
            return Err(ApiError::error("Failed to get file from request"));
        }
    };

    if field.content_type().essence_str() != "text/csv" {
        return Err(ApiError::error("File's Content-Type was not text/csv"));
    }

    let data: Result<Vec<web::Bytes>, _> = field.try_collect().await;
    let bytes: bytes::Bytes = data?.into_iter().flatten().collect();
    let reader = bytes.reader();
    let mut rdr = csv::Reader::from_reader(reader);

    let result: Result<(Vec<NewFermentable>, Vec<csv::Error>), BlockingError> =
        web::block(move || {
            let (fermentables, errors): (Vec<_>, Vec<_>) =
                rdr.deserialize().partition(Result::is_ok);
            let numbers: Vec<_> = fermentables.into_iter().map(Result::unwrap).collect();
            let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
            (numbers, errors)
        })
        .await;

    let (new_fermentables, errors) = result.unwrap();

    if errors.len() > 0 {
        tracing::info!("errors {:?}", errors);
        Err(ApiError::error(format!("{:?}", errors)))
    } else {
        Fermentable::bulk_insert(db.get_ref(), new_fermentables).await?;
        Ok(Json(()))
    }
}

#[tracing::instrument(skip(db))]
pub async fn list(db: Data<PgPool>) -> Result<Json<Vec<Fermentable>>, ApiError> {
    let fermentables = Fermentable::list(db.get_ref()).await?;
    Ok(Json(fermentables))
}

#[tracing::instrument(skip(db))]
pub async fn delete(
    db: Data<PgPool>,
    fermentable_id: Json<FermentableId>,
) -> Result<Json<()>, ApiError> {
    Fermentable::delete(db.get_ref(), fermentable_id.0).await?;
    Ok(Json(()))
}

#[tracing::instrument(skip(db))]
pub async fn search(
    db: Data<PgPool>,
    search: Json<FermentableSearch>,
) -> Result<Json<Vec<Fermentable>>, ApiError> {
    let rows = Fermentable::search(db.get_ref(), search.0).await?;
    tracing::info!("row: {:?}", rows.len());
    Ok(Json(rows))
}

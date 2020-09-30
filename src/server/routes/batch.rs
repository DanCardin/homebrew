use crate::error::UserError;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use chrono::NaiveDate;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBatch {
    pub beer_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdate {
    pub batch_id: i32,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    pub id: i32,
    pub beer_id: i32,
    pub date: NaiveDate,
}

impl Batch {
    pub async fn insert(db: &PgPool, new_batch: NewBatch) -> Result<Self, UserError> {
        let trans = db.begin().await?;

        let beer = sqlx::query_as!(
            Batch,
            "INSERT INTO batch (beer_id) VALUES ($1) RETURNING id, beer_id, date",
            new_batch.beer_id
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(beer)
    }

    pub async fn find(db: &PgPool, batch_id: i32) -> Result<Self, UserError> {
        let batch = sqlx::query_as!(
            Batch,
            "SELECT id, beer_id, date FROM batch WHERE id = $1",
            batch_id
        )
        .fetch_one(db)
        .await?;
        Ok(batch)
    }

    pub async fn list_for_beer(db: &PgPool, beer_id: i32) -> Result<Vec<Self>, UserError> {
        let batches = sqlx::query_as!(
            Batch,
            "SELECT id, beer_id, date FROM batch WHERE beer_id = $1 ORDER BY date DESC",
            beer_id
        )
        .fetch_all(db)
        .await?;
        Ok(batches)
    }

    pub async fn update_date(
        db: &PgPool,
        batch_id: i32,
        date: NaiveDate,
    ) -> Result<Self, UserError> {
        let trans = db.begin().await?;

        let beer = sqlx::query_as!(
            Batch,
            "UPDATE batch SET date = $1 WHERE id = $2 RETURNING id, beer_id, date",
            date,
            batch_id,
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(beer)
    }

    pub async fn delete(db: &PgPool, batch_id: i32) -> Result<(), UserError> {
        let trans = db.begin().await?;

        sqlx::query_as!(Batch, "DELETE FROM batch WHERE id = $1", batch_id)
            .execute(db)
            .await?;

        trans.commit().await?;
        Ok(())
    }
}

pub async fn new_batch(
    db: Data<PgPool>,
    new_batch: Json<NewBatch>,
) -> Result<HttpResponse, UserError> {
    let batch = Batch::insert(db.get_ref(), new_batch.0).await?;
    info!("{:?}", batch);
    Ok(HttpResponse::Ok().json(batch))
}

pub async fn list_batches(
    db: Data<PgPool>,
    new_batch: Json<NewBatch>,
) -> Result<HttpResponse, UserError> {
    let batches = Batch::list_for_beer(db.get_ref(), new_batch.0.beer_id).await?;
    info!("{:?}", batches);
    Ok(HttpResponse::Ok().json(batches))
}

pub async fn update_batch_date(
    db: Data<PgPool>,
    batch_update: Json<BatchUpdate>,
) -> Result<HttpResponse, UserError> {
    let batch_id = batch_update.0.batch_id;

    let db = db.get_ref();
    let batch = if let Some(date) = batch_update.0.date {
        Batch::update_date(db, batch_id, date).await?
    } else {
        Batch::find(db, batch_id).await?
    };
    info!("{:?}", batch);
    Ok(HttpResponse::Ok().json(batch))
}

pub async fn delete_batch(
    db: Data<PgPool>,
    batch_update: Json<BatchUpdate>,
) -> Result<HttpResponse, UserError> {
    let batch_id = batch_update.0.batch_id;
    Batch::delete(db.get_ref(), batch_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

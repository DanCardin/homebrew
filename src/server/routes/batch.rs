use crate::error::ApiError;
use actix_web::web::{Data, Json};
use chrono::NaiveDate;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;
use tracing;
use tracing::info;

pub mod ingredient;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBatch {
    pub beer_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchId {
    pub batch_id: i32,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measurement {
    pub batch_id: i32,
    pub name: String,
    pub value: Option<f64>,
}

#[derive(Debug, sqlx::FromRow)]
struct DbMeasurement {
    pub batch_id: i32,
    pub name: String,
    pub value: sqlx::types::BigDecimal,
}

impl From<DbMeasurement> for Measurement {
    fn from(m: DbMeasurement) -> Self {
        Self {
            batch_id: m.batch_id,
            name: m.name,
            value: m.value.to_f64(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchInfo {
    pub measurements: Vec<Measurement>,
}

impl Batch {
    pub async fn insert(db: &PgPool, new_batch: NewBatch) -> Result<Self, ApiError> {
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

    pub async fn find(db: &PgPool, batch_id: i32) -> Result<Self, ApiError> {
        let batch = sqlx::query_as!(
            Batch,
            "SELECT id, beer_id, date FROM batch WHERE id = $1",
            batch_id
        )
        .fetch_one(db)
        .await?;
        Ok(batch)
    }

    pub async fn list_for_beer(db: &PgPool, beer_id: i32) -> Result<Vec<Self>, ApiError> {
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
    ) -> Result<Self, ApiError> {
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

    pub async fn delete(db: &PgPool, batch_id: i32) -> Result<(), ApiError> {
        let trans = db.begin().await?;

        sqlx::query_as!(Batch, "DELETE FROM batch WHERE id = $1", batch_id)
            .execute(db)
            .await?;

        trans.commit().await?;
        Ok(())
    }

    pub async fn update_measurement(
        db: &PgPool,
        batch_id: i32,
        name: String,
        value: f64,
    ) -> Result<Measurement, ApiError> {
        let trans = db.begin().await?;

        let measurement = sqlx::query_as!(
            DbMeasurement,
            r#"
            INSERT INTO batch_measurement (batch_id, name, value)
            VALUES ($1, $2, $3)
            ON CONFLICT
            ON CONSTRAINT batch_measurement_pkey
            DO UPDATE SET value = $3
            RETURNING batch_id, name, value
            "#,
            batch_id,
            name,
            sqlx::types::BigDecimal::from(value)
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(measurement.into())
    }

    pub async fn get_info(db: &PgPool, batch_id: i32) -> Result<BatchInfo, ApiError> {
        let db_measurements = sqlx::query_as!(
            DbMeasurement,
            r#"
            SELECT batch_id, name, value
            FROM batch_measurement
            WHERE batch_id = $1
            "#,
            batch_id,
        )
        .fetch_all(db)
        .await?;

        let measurements: Vec<Measurement> =
            db_measurements.into_iter().map(|row| row.into()).collect();
        Ok(BatchInfo { measurements })
    }
}

pub async fn new_batch(
    db: Data<PgPool>,
    new_batch: Json<NewBatch>,
) -> Result<Json<Batch>, ApiError> {
    let batch = Batch::insert(db.get_ref(), new_batch.0).await?;
    info!("{:?}", batch);
    Ok(Json(batch))
}

#[tracing::instrument(
    skip(db, new_batch),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn list_batches(
    db: Data<PgPool>,
    new_batch: Json<NewBatch>,
) -> Result<Json<Vec<Batch>>, ApiError> {
    let batches = Batch::list_for_beer(db.get_ref(), new_batch.0.beer_id).await?;
    info!("{:?}", batches);
    Ok(Json(batches))
}

pub async fn update_batch_date(
    db: Data<PgPool>,
    batch_update: Json<BatchUpdate>,
) -> Result<Json<Batch>, ApiError> {
    let batch_id = batch_update.0.batch_id;

    let db = db.get_ref();
    let batch = if let Some(date) = batch_update.0.date {
        Batch::update_date(db, batch_id, date).await?
    } else {
        Batch::find(db, batch_id).await?
    };
    info!("{:?}", batch);
    Ok(Json(batch))
}

pub async fn delete_batch(
    db: Data<PgPool>,
    batch_update: Json<BatchUpdate>,
) -> Result<Json<()>, ApiError> {
    let batch_id = batch_update.0.batch_id;
    Batch::delete(db.get_ref(), batch_id).await?;
    Ok(Json(()))
}

#[tracing::instrument(
    skip(db, measurement),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn update_batch_measurement(
    db: Data<PgPool>,
    measurement: Json<Measurement>,
) -> Result<Json<Measurement>, ApiError> {
    let m = measurement.0;
    let measurement = if let Some(value) = m.value {
        Batch::update_measurement(db.get_ref(), m.batch_id, m.name, value).await?
    } else {
        m
    };
    info!("{:?}", measurement);
    Ok(Json(measurement))
}

#[tracing::instrument(
    skip(db, batch_id),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn get_batch_info(
    db: Data<PgPool>,
    batch_id: Json<BatchId>,
) -> Result<Json<BatchInfo>, ApiError> {
    let batch_info = Batch::get_info(db.get_ref(), batch_id.0.batch_id).await?;
    info!("{:?}", batch_info);
    Ok(Json(batch_info))
}

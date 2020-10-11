use crate::error::ApiError;
use actix_multipart::Multipart;
use actix_web::error::BlockingError;
use actix_web::web;
use actix_web::web::{Data, Json};
use bytes::buf::BufExt;
use futures::TryStreamExt;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use tracing;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FermentableId {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFermentable {
    pub name: String,
    pub country: Option<String>,
    pub category: String,
    pub kind: String,
    pub color: i32,
    pub ppg: f64,
}

#[derive(Debug, Serialize)]
pub struct Fermentable {
    pub id: i32,
    pub name: String,
    pub country: Option<String>,
    pub category: String,
    pub kind: String,
    pub color: i32,
    pub ppg: f64,
}

#[derive(Debug, sqlx::FromRow)]
struct DbFermentable {
    pub id: i32,
    pub name: String,
    pub country: Option<String>,
    pub category: String,
    pub kind: String,
    pub color: i32,
    pub ppg: sqlx::types::BigDecimal,
}

impl From<DbFermentable> for Fermentable {
    fn from(m: DbFermentable) -> Self {
        Self {
            id: m.id,
            name: m.name,
            country: m.country,
            category: m.category,
            kind: m.kind,
            color: m.color,
            ppg: m.ppg.to_f64().unwrap_or(0.),
        }
    }
}

impl Fermentable {
    pub async fn insert(db: &PgPool, new: NewFermentable) -> Result<Self, ApiError> {
        let trans = db.begin().await?;

        let row = sqlx::query_as!(
            DbFermentable,
            r#"
            INSERT INTO fermentable (name, country, category, kind, color, ppg)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, country, category, kind, color, ppg
            "#,
            new.name,
            new.country,
            new.category,
            new.kind,
            new.color,
            sqlx::types::BigDecimal::from(new.ppg)
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(row.into())
    }

    pub async fn bulk_insert(
        db: &PgPool,
        new_fermentables: Vec<NewFermentable>,
    ) -> Result<(), ApiError> {
        let trans = db.begin().await?;

        fn collect(
            rows: Vec<NewFermentable>,
        ) -> (
            Vec<String>,
            Vec<Option<String>>,
            Vec<String>,
            Vec<String>,
            Vec<i32>,
            Vec<sqlx::types::BigDecimal>,
        ) {
            let mut names = Vec::new();
            let mut countries = Vec::new();
            let mut categories = Vec::new();
            let mut kinds = Vec::new();
            let mut colors = Vec::new();
            let mut ppgs = Vec::new();
            for row in rows.into_iter() {
                names.push(row.name);
                countries.push(row.country);
                categories.push(row.category);
                kinds.push(row.kind);
                colors.push(row.color);
                ppgs.push(sqlx::types::BigDecimal::from(row.ppg));
            }
            (names, countries, categories, kinds, colors, ppgs)
        }

        let (names, countries, categories, kinds, colors, ppgs) = collect(new_fermentables);
        sqlx::query_as!(
            Fermentable,
            r#"
            INSERT INTO fermentable (name, country, category, kind, color, ppg)
            VALUES (
                UNNEST($1::VARCHAR[]),
                UNNEST($2::VARCHAR[]),
                UNNEST($3::VARCHAR[]),
                UNNEST($4::VARCHAR[]),
                UNNEST($5::INTEGER[]),
                UNNEST($6::DECIMAL[])
            )"#,
            &names,
            &countries as _,
            &categories,
            &kinds,
            &colors,
            &ppgs
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(())
    }

    pub async fn delete(db: &PgPool, id: i32) -> Result<(), ApiError> {
        let trans = db.begin().await?;

        sqlx::query_as!(DbFermentable, "DELETE FROM fermentable WHERE id = $1", id)
            .execute(db)
            .await?;

        trans.commit().await?;
        Ok(())
    }

    pub async fn list(db: &PgPool) -> Result<Vec<Self>, ApiError> {
        let rows = sqlx::query_as!(
            DbFermentable,
            "SELECT id, name, country, category, kind, color, ppg FROM fermentable",
        )
        .fetch_all(db)
        .await?;

        let rows: Vec<Fermentable> = rows.into_iter().map(|row| row.into()).collect();
        Ok(rows)
    }
}

#[tracing::instrument(
    skip(db),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn new(
    db: Data<PgPool>,
    new_fermentable: Json<NewFermentable>,
) -> Result<Json<Fermentable>, ApiError> {
    let fermentable = Fermentable::insert(db.get_ref(), new_fermentable.0).await?;
    Ok(Json(fermentable))
}

#[tracing::instrument(
    skip(db, payload),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
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

    let result: Result<(Vec<NewFermentable>, Vec<csv::Error>), BlockingError<Infallible>> =
        web::block(move || {
            let (fermentables, errors): (Vec<_>, Vec<_>) =
                rdr.deserialize().partition(Result::is_ok);
            let numbers: Vec<_> = fermentables.into_iter().map(Result::unwrap).collect();
            let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
            Ok((numbers, errors))
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

#[tracing::instrument(
    skip(db),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn list(db: Data<PgPool>) -> Result<Json<Vec<Fermentable>>, ApiError> {
    let fermentables = Fermentable::list(db.get_ref()).await?;
    Ok(Json(fermentables))
}

#[tracing::instrument(
    skip(db, fermentable_id),
    fields(
        request_id=%uuid::Uuid::new_v4(),
    )
)]
pub async fn delete(
    db: Data<PgPool>,
    fermentable_id: Json<FermentableId>,
) -> Result<Json<()>, ApiError> {
    Fermentable::delete(db.get_ref(), fermentable_id.0.id).await?;
    Ok(Json(()))
}

use crate::error::ApiError;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;
use std::convert::TryFrom;

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

#[derive(Debug, Deserialize)]
pub struct FermentableSearch {
    pub query: Option<String>,
    pub ids: Option<Vec<i32>>,
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
            sqlx::types::BigDecimal::try_from(new.ppg).unwrap()
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
                ppgs.push(sqlx::types::BigDecimal::try_from(row.ppg).unwrap());
            }
            (names, countries, categories, kinds, colors, ppgs)
        }

        let (names, countries, categories, kinds, colors, ppgs) = collect(new_fermentables);
        sqlx::query!(
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

    pub async fn delete(db: &PgPool, fermentable_id: FermentableId) -> Result<(), ApiError> {
        let trans = db.begin().await?;

        sqlx::query_as!(
            DbFermentable,
            "DELETE FROM fermentable WHERE id = $1",
            fermentable_id.id
        )
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

    pub async fn search(db: &PgPool, search: FermentableSearch) -> Result<Vec<Self>, ApiError> {
        let query = format!("%{}%", search.query.unwrap_or("<null>".to_string()));
        let ids = match search.ids {
            Some(ids) => ids,
            None => Vec::new(),
        };

        let rows = sqlx::query_as!(
            DbFermentable,
            r#"
            SELECT id, name, country, category, kind, color, ppg
            FROM fermentable
            WHERE (
                CONCAT(
                    name, ',', country, ',', category, ',',
                    kind, ',', color, ',', ppg
                ) ILIKE $1
                OR
                id = any($2)
            )
            "#,
            query,
            &ids,
        )
        .fetch_all(db)
        .await?;

        let rows: Vec<Fermentable> = rows.into_iter().map(|row| row.into()).collect();
        Ok(rows)
    }
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

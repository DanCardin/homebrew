use crate::error::ApiError;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;
use tracing::info;

pub struct NewBeer {
    pub id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct Beer {
    pub id: i32,
    pub name: String,
    pub style: String,
}

#[derive(Deserialize)]
pub struct BeerRequest {
    id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct BeerUpdate {
    id: i32,
    name: Option<String>,
    style: Option<String>,
}

impl Beer {
    pub async fn insert(db: &PgPool) -> Result<Beer, ApiError> {
        let trans = db.begin().await?;

        let beer = sqlx::query_as!(
            Beer,
            "INSERT INTO beer DEFAULT VALUES RETURNING id, name, style"
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(beer)
    }

    pub async fn find(db: &PgPool, id: i32) -> Result<Self, ApiError> {
        Ok(sqlx::query_as!(
            Beer,
            "SELECT id, name, style FROM beer WHERE id = $1 ORDER BY id DESC",
            id,
        )
        .fetch_one(db)
        .await?)
    }

    pub async fn all(db: &PgPool) -> Result<Vec<Self>, ApiError> {
        Ok(
            sqlx::query_as!(Beer, "SELECT id, name, style FROM beer ORDER BY id DESC")
                .fetch_all(db)
                .await?,
        )
    }

    pub async fn update(db: &PgPool, updated_beer: BeerUpdate) -> Result<Beer, ApiError> {
        let name = updated_beer.name.unwrap_or("".to_string());
        let style = updated_beer.style.unwrap_or("".to_string());

        let trans = db.begin().await?;

        let beer = sqlx::query_as!(
            Beer,
            "UPDATE beer SET name = $1, style = $2 WHERE id = $3 RETURNING id, name, style",
            name,
            style,
            updated_beer.id,
        )
        .fetch_one(db)
        .await?;

        trans.commit().await?;
        Ok(beer)
    }
}

pub async fn new_beer(db: Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let beer = Beer::insert(db.get_ref()).await?;
    info!("{:?}", beer);
    Ok(HttpResponse::Ok().json(Beer {
        id: beer.id,
        name: "".to_string(),
        style: "".to_string(),
    }))
}

pub async fn get_beer(
    db: Data<PgPool>,
    beer_request: Json<BeerRequest>,
) -> Result<HttpResponse, ApiError> {
    let beer = Beer::find(db.get_ref(), beer_request.0.id).await?;
    Ok(HttpResponse::Ok().json(beer))
}

pub async fn list_beers(db: Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let beers: Vec<Beer> = Beer::all(db.get_ref()).await?;
    info!("{:?}", beers);
    Ok(HttpResponse::Ok().json(beers))
}

pub async fn update_beer(
    db: Data<PgPool>,
    beer_update: Json<BeerUpdate>,
) -> Result<HttpResponse, ApiError> {
    let updated_beer = Beer::update(db.get_ref(), beer_update.0).await?;
    info!("{:?}", updated_beer);
    Ok(HttpResponse::Ok().json(BeerUpdate {
        id: updated_beer.id,
        name: Some(updated_beer.name),
        style: Some(updated_beer.style),
    }))
}

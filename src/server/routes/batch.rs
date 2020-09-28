use crate::{schema::batch, Db};
use chrono::NaiveDate;
use diesel::prelude::*;
use log::info;
use rocket::post;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Deserialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "batch"]
pub struct NewBatch {
    pub beer_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdate {
    pub batch_id: i32,
    pub date: Option<NaiveDate>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    pub id: i32,
    pub beer_id: i32,
    pub date: NaiveDate,
}

impl Batch {
    pub fn insert(conn: &mut SqliteConnection, new_batch: NewBatch) {
        diesel::insert_into(batch::table)
            .values(&new_batch)
            .execute(conn)
            .expect("error");
    }

    pub fn find_last(conn: &mut SqliteConnection) -> Self {
        batch::table
            .order(batch::id.desc())
            .first(conn)
            .expect("error")
    }

    pub fn find(conn: &mut SqliteConnection, batch_id: i32) -> Self {
        batch::table
            .filter(batch::id.eq(batch_id))
            .first(conn)
            .expect("error")
    }

    pub fn all(conn: &mut SqliteConnection) -> Vec<Self> {
        batch::table
            .order(batch::date.desc())
            .get_results(conn)
            .expect("error")
    }

    pub fn list_for_batch(conn: &mut SqliteConnection, beer_id: i32) -> Vec<Self> {
        batch::table
            .filter(batch::beer_id.eq(beer_id))
            .order(batch::date.desc())
            .get_results(conn)
            .expect("error")
    }

    pub fn update_date(conn: &mut SqliteConnection, batch_id: i32, date: NaiveDate) -> Self {
        diesel::update(batch::table.filter(batch::id.eq(batch_id)))
            .set(batch::date.eq(date))
            .execute(conn)
            .expect("error");

        batch::table
            .filter(batch::id.eq(batch_id))
            .first(conn)
            .expect("error")
    }

    pub fn delete(conn: &mut SqliteConnection, batch_id: i32) {
        diesel::delete(batch::table.filter(batch::id.eq(batch_id)))
            .execute(conn)
            .expect("error");
    }
}

#[post("/beer.batch.new", format = "json", data = "<new_batch>")]
pub async fn create_batch(db: Db, new_batch: Json<NewBatch>) -> Json<Batch> {
    let batch: Batch = db
        .run(|conn| {
            Batch::insert(conn, new_batch.0);
            Batch::find_last(conn)
        })
        .await;
    info!("{:?}", batch);
    Json(batch)
}

#[post("/beer.batch.list", format = "json", data = "<new_batch>")]
pub async fn list_batches(db: Db, new_batch: Json<NewBatch>) -> Json<Vec<Batch>> {
    let batches: Vec<Batch> = db
        .run(move |conn| Batch::list_for_batch(conn, new_batch.0.beer_id))
        .await;

    info!("{:?}", batches);
    Json(batches)
}

#[post("/beer.batch.date.update", format = "json", data = "<batch_update>")]
pub async fn update_batch_date(db: Db, batch_update: Json<BatchUpdate>) -> Json<Batch> {
    let batch_id = batch_update.0.batch_id;

    let batch: Batch = db
        .run(move |conn| {
            if let Some(date) = batch_update.0.date {
                Batch::update_date(conn, batch_id, date)
            } else {
                Batch::find(conn, batch_id)
            }
        })
        .await;

    info!("{:?}", batch);
    Json(batch)
}

#[post("/beer.batch.date.delete", format = "json", data = "<batch_update>")]
pub async fn delete_batch(db: Db, batch_update: Json<BatchUpdate>) {
    let batch_id = batch_update.0.batch_id;

    db.run(move |conn| {
        Batch::delete(conn, batch_id);
    })
    .await;
}

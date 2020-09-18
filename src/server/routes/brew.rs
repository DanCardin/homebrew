use crate::{schema::brew, Db};
use diesel::prelude::*;
use log::info;
use rocket::post;
use rocket_contrib::json::Json;
use serde::Serialize;
use chrono::NaiveDate;

#[derive(Insertable)]
#[table_name="brew"]
pub struct NewBrew {
    pub id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct Brew {
    pub id: i32,
    pub name: String,
    pub style: String,
    pub date: Option<NaiveDate>,
    pub batch: i32,
}

impl Brew {
    pub fn insert(conn: &mut SqliteConnection, new_brew: NewBrew) {
            diesel::insert_into(brew::table)
                .values(&new_brew)
                .execute(conn)
                .expect("error");
    }

    pub fn find_last(conn: &mut SqliteConnection) -> Self {
        brew::table
            .order(brew::id.desc())
            .first(conn)
            .expect("error")
    }

    pub fn all(conn: &mut SqliteConnection) -> Vec<Self> {
        brew::table
            .order(brew::id.desc())
            .get_results(conn)
            .expect("error")
    }
} 


#[derive(Serialize)]
pub struct BrewResponse {
    id: i32,
}

#[post("/brew.new", format = "json")]
pub async fn new_brew(db: Db) -> Json<BrewResponse> {
    let new_brew = NewBrew { id: None};
    let brew: Brew = db
        .run(|conn| {
            Brew::insert(conn, new_brew);
            Brew::find_last(conn)
        })
        .await;
    info!("{:?}", brew);
    Json(BrewResponse { id: brew.id })
}

#[post("/brew.list", format = "json")]
pub async fn list_brews(db: Db) -> Json<Vec<BrewResponse>> {
    let brews: Vec<Brew> = db
        .run(|conn| {
            Brew::all(conn)
        })
        .await;
    info!("{:?}", brews);

    Json(brews.iter().map(|brew| BrewResponse { id: brew.id }).collect())
}

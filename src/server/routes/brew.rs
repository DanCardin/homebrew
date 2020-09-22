use crate::{schema::brew, Db};
use diesel::prelude::*;
use log::info;
use rocket::post;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
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

    // pub fn update(conn: &mut SqliteConnection, updated_brew: BrewUpdate) {
    //     let mut query = diesel::update(brew::table.filter(brew::id.eq(updated_brew.id))).into_boxed();
    //
    //     if let Some(name) = updated_brew.name {
    //         query = query.set(brew::name.eq(name));
    //     }
    //
    //     query.execute(conn).expect("error");
    // }

    pub fn find(conn: &mut SqliteConnection, id: i32) -> Self {
        brew::table
            .order(brew::id.desc())
            .filter(brew::id.eq(id))
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

#[derive(Deserialize, Serialize)]
pub struct BrewUpdate {
    id: i32,
    name: Option<String>,
    style: Option<String>,
    batch: Option<i32>,
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

#[post("/brew.update", format = "json", data = "<brew_update>")]
pub async fn update_brew(db: Db, brew_update: Json<BrewUpdate>) -> Json<BrewUpdate> {
    // let updated_brew: Brew = db
    //     .run(|conn| {
    //         // let id = brew_update.0.id.clone();
    //         // Brew::update(conn, brew_update.0);
    //         // Brew::find(conn, id)
    //     })
    //     .await;
    // info!("{:?}", updated_brew);
    // Json(BrewUpdate { id: updated_brew.id, name: Some(updated_brew.name), batch: Some(updated_brew.batch), style: Some(updated_brew.style) })
    Json(BrewUpdate { id: brew_update.id, name: None, batch: None, style: None })
}

use crate::{schema::beer, Db};
use diesel::prelude::*;
use log::info;
use rocket::post;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name = "beer"]
pub struct NewBeer {
    pub id: Option<i32>,
}

#[derive(Queryable, Debug, Serialize)]
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
    pub fn insert(conn: &mut SqliteConnection, new_beer: NewBeer) {
        diesel::insert_into(beer::table)
            .values(&new_beer)
            .execute(conn)
            .expect("error");
    }

    pub fn find_last(conn: &mut SqliteConnection) -> Self {
        beer::table
            .order(beer::id.desc())
            .first(conn)
            .expect("error")
    }

    pub fn update(conn: &mut SqliteConnection, updated_beer: BeerUpdate) {
        let name = updated_beer.name.unwrap_or("".to_string());
        let style = updated_beer.style.unwrap_or("".to_string());
        let query = diesel::update(beer::table.filter(beer::id.eq(updated_beer.id)))
            .set((beer::name.eq(name), beer::style.eq(style)));
        query.execute(conn).expect("error");
    }

    pub fn find(conn: &mut SqliteConnection, id: i32) -> Self {
        beer::table
            .order(beer::id.desc())
            .filter(beer::id.eq(id))
            .first(conn)
            .expect("error")
    }

    pub fn all(conn: &mut SqliteConnection) -> Vec<Self> {
        beer::table
            .order(beer::id.desc())
            .get_results(conn)
            .expect("error")
    }
}

#[post("/beer.new", format = "json")]
pub async fn new_beer(db: Db) -> Json<Beer> {
    let new_beer = NewBeer { id: None };
    let beer: Beer = db
        .run(|conn| {
            Beer::insert(conn, new_beer);
            Beer::find_last(conn)
        })
        .await;
    info!("{:?}", beer);
    Json(Beer {
        id: beer.id,
        name: "".to_string(),
        style: "".to_string(),
    })
}

#[post("/beer.list", format = "json")]
pub async fn list_beers(db: Db) -> Json<Vec<Beer>> {
    let beers: Vec<Beer> = db.run(|conn| Beer::all(conn)).await;
    info!("{:?}", beers);

    Json(beers)
}

#[post("/beer.update", format = "json", data = "<beer_update>")]
pub async fn update_beer(db: Db, beer_update: Json<BeerUpdate>) -> Json<BeerUpdate> {
    let updated_beer: Beer = db
        .run(|conn| {
            let id = beer_update.0.id.clone();
            Beer::update(conn, beer_update.0);
            Beer::find(conn, id)
        })
        .await;
    info!("{:?}", updated_beer);
    Json(BeerUpdate {
        id: updated_beer.id,
        name: Some(updated_beer.name),
        style: Some(updated_beer.style),
    })
}

#[post("/beer.get", format = "json", data = "<beer_request>")]
pub async fn get_beer(db: Db, beer_request: Json<BeerRequest>) -> Json<Beer> {
    let beer: Beer = db
        .run(move |conn| Beer::find(conn, beer_request.0.id))
        .await;
    Json(beer)
}

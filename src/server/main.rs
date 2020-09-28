use homebrew::srm::SRM_TO_HEX;
use log;
use rocket::{fairing::AdHoc, ignite, launch, post, routes, FromForm, Rocket};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod routes;
pub mod schema;

#[rocket_contrib::database("sqlite")]
pub struct Db(diesel::SqliteConnection);

#[derive(FromForm, Deserialize)]
struct SrmRequest {
    value: u8,
}

#[derive(Serialize)]
struct SrmResponse {
    value: String,
}

#[post("/srm.convert", format = "json", data = "<srm>")]
fn srm_convert(srm: Json<SrmRequest>) -> Json<SrmResponse> {
    Json(SrmResponse {
        value: SRM_TO_HEX.get(&srm.value).unwrap_or(&"#000000").to_string(),
    })
}

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`.
embed_migrations!();

async fn run_migrations(mut rocket: Rocket) -> Result<Rocket, Rocket> {
    Db::get_one(rocket.inspect().await)
        .await
        .expect("database connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                log::error!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
}

#[launch]
fn configure() -> Rocket {
    ignite()
        .attach(Db::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_migrations))
        .mount(
            "/",
            routes![
                srm_convert,
                routes::beer::get_beer,
                routes::beer::list_beers,
                routes::beer::new_beer,
                routes::beer::update_beer,
                routes::batch::create_batch,
                routes::batch::delete_batch,
                routes::batch::list_batches,
                routes::batch::update_batch_date,
            ],
        )
}

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use listenfd::ListenFd;
use sqlx::postgres::PgPool;

pub mod config;
pub mod error;
pub mod routes;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let configuration = config::get_config().expect("Failed to read configuration.");

    let mut listenfd = ListenFd::from_env();
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let connection = web::Data::new(connection);
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(connection.clone())
            .route("/health", web::get().to(routes::check_health))
            .route("/beer.get", web::post().to(routes::beer::get_beer))
            .route("/beer.list", web::post().to(routes::beer::list_beers))
            .route("/beer.new", web::post().to(routes::beer::new_beer))
            .route("/beer.update", web::post().to(routes::beer::update_beer))
            .route("/beer.batch.new", web::post().to(routes::batch::new_batch))
            .route(
                "/beer.batch.list",
                web::post().to(routes::batch::list_batches),
            )
            .route(
                "/beer.batch.date.update",
                web::post().to(routes::batch::update_batch_date),
            )
            .route(
                "/beer.batch.delete",
                web::post().to(routes::batch::delete_batch),
            )
    })
    .workers(4);

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let port = 8000;
            server.bind(format!("127.0.0.1:{}", port))?
        }
    };

    server.run().await
}

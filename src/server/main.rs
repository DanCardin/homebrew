use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;
use sqlx::postgres::PgPool;
use tracing_actix_web::TracingLogger;

pub mod config;
pub mod error;
pub mod routes;
pub mod telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

    let subscriber = telemetry::get_subscriber("info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = config::get_config().expect("Failed to read configuration.");

    let mut listenfd = ListenFd::from_env();
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let connection = web::Data::new(connection);
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .app_data(connection.clone())
            .route("/health", web::get().to(routes::check_health))
            .service(
                web::scope("/measurement")
                    .route(
                        "abv/calculate",
                        web::post().to(routes::measurement::abv::calculate_abv),
                    )
                    .route(
                        "srm/to_hex",
                        web::post().to(routes::measurement::srm::to_hex),
                    ),
            )
            .service(
                web::scope("/beer")
                    .route("get", web::post().to(routes::beer::get_beer))
                    .route("list", web::post().to(routes::beer::list_beers))
                    .route("new", web::post().to(routes::beer::new_beer))
                    .route("update", web::post().to(routes::beer::update_beer))
                    .service(
                        web::scope("batch")
                            .route("new", web::post().to(routes::batch::new_batch))
                            .route("list", web::post().to(routes::batch::list_batches))
                            .route("get", web::post().to(routes::batch::get_batch_info))
                            .route("delete", web::post().to(routes::batch::delete_batch))
                            .route(
                                "date/update",
                                web::post().to(routes::batch::update_batch_date),
                            )
                            .route(
                                "measurement/update",
                                web::post().to(routes::batch::update_batch_measurement),
                            )
                            .service(
                                web::scope("ingredient")
                                    .route("new", web::post().to(routes::batch::ingredient::new))
                                    .route("list", web::post().to(routes::batch::ingredient::list))
                                    .route(
                                        "delete",
                                        web::post().to(routes::batch::ingredient::delete),
                                    ),
                            ),
                    ),
            )
            .service(
                web::scope("/fermentable")
                    .route("new", web::post().to(routes::fermentable::new))
                    .route("import", web::post().to(routes::fermentable::import))
                    .route("delete", web::post().to(routes::fermentable::delete))
                    .route("list", web::post().to(routes::fermentable::list)),
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

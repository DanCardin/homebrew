use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use listenfd::ListenFd;
use sqlx::postgres::PgPool;
use tracing_actix_web::TracingLogger;

pub mod config;
pub mod error;
pub mod repos;
pub mod routes;
pub mod telemetry;

fn json_error_handler(
    err: actix_web::error::JsonPayloadError,
    _req: &HttpRequest,
) -> actix_web::error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    actix_web::error::InternalError::from_response(err, resp).into()
}

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
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
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
                                web::scope("fermentable")
                                    .route("new", web::post().to(routes::batch::fermentable::new))
                                    .route(
                                        "delete",
                                        web::post().to(routes::batch::fermentable::delete),
                                    )
                                    .route("list", web::post().to(routes::batch::fermentable::list))
                                    .route(
                                        "update",
                                        web::post().to(routes::batch::fermentable::update),
                                    )
                                    .route(
                                        "delete",
                                        web::post().to(routes::batch::fermentable::delete),
                                    ),
                            ),
                    ),
            )
            .service(
                web::scope("/fermentable")
                    .route("new", web::post().to(routes::fermentable::new))
                    .route("import", web::post().to(routes::fermentable::import))
                    .route("delete", web::post().to(routes::fermentable::delete))
                    .route("list", web::post().to(routes::fermentable::list))
                    .route("search", web::post().to(routes::fermentable::search)),
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

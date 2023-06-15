#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::env;

use actix_cors::Cors;
use actix_rt;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use futures_util::lock::Mutex;
use uuid::Uuid;

mod routes;
mod websocket;

pub struct AppState {
    users: Mutex<HashMap<Uuid, String>>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let state = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(
                &env::var("CLIENT_HOST").expect("No `CLIENT_HOST` on environment variables!"),
            )
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);
        // Allow all origins CORS.
        let cors = Cors::default().allow_any_origin();
        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

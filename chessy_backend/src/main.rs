#[macro_use]
extern crate log;

use std::env;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use actix::Actor;
use actix_cors::Cors;
use actix_rt;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use uuid::Uuid;

mod game;
mod player;
mod routes;
mod websocket;

type AppState = Mutex<HashMap<Uuid, Arc<str>>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let state: Arc<AppState> = Arc::default();

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
        let cors = Cors::default().allow_any_origin().allow_any_header();
        App::new()
            .app_data(web::Data::from(state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

mod rest_api;
use actix_web::web;

pub use self::rest_api::*;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").route("/register", web::post().to(rest_api::register_user)));
    cfg.service(web::scope("/game").route("/new", web::post().to(rest_api::new_game)));
}

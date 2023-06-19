mod rest_api;
mod ws;

use actix_web::web;

pub use self::rest_api::*;
pub use self::ws::*;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").route("/register", web::post().to(rest_api::register_user)));
    cfg.service(web::resource("/ws/{client_id}").to(ws::ws_endpoint));
}

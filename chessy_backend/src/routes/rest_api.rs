use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

pub async fn register_user(name: String, data: web::Data<AppState>) -> impl Responder {
    log::debug!("Received username {}", &name);
    let id = Uuid::new_v4();
    log::debug!("Generated UUID ({}) for user", &id);

    let mut users = data.users.lock().await;
    users.insert(id, name);
    log::debug!("The current hashmap of users is {:?}", users);

    HttpResponse::Ok().message_body(id.hyphenated().to_string())
}

#[derive(Serialize)]
pub struct NewGameResponse {
    #[serde(alias = "socketUrl")]
    socket_url: String,
    #[serde(alias = "gameId")]
    game_id: String,
}

pub async fn new_game(userId: String, data: web::Data<AppState>) -> impl Responder {
    log::debug!("User ({}) want's to create a new game!", &userId);

    let socket_url = String::new();
    let game_id = String::new();

    HttpResponse::Ok().json(NewGameResponse {
        socket_url,
        game_id,
    })
}

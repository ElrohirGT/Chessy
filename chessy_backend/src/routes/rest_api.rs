use std::fmt::Display;

use actix_web::{web, HttpResponse, Responder, ResponseError, http::header::ContentType};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug)]
pub enum RegisterErrors {
    NameIsEmpty
}

impl Display for RegisterErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RegisterErrors::NameIsEmpty => f.write_str("The user name is empty!"),
        }
    }
}

impl ResponseError for RegisterErrors {
    fn status_code(&self) -> actix_http::StatusCode {
        actix_http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse<actix_http::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }

}

pub async fn register_user(name: String, data: web::Data<AppState>) -> Result<String, RegisterErrors> {
    log::debug!("Received username `{}`", &name);

    if name.is_empty() {
        return Err(RegisterErrors::NameIsEmpty);
    }

    let id = Uuid::new_v4();
    log::debug!("Generated UUID `{}` for user `{}`", &id, &name);

    let mut users = data.users.lock().await;
    users.insert(id, name);
    log::debug!("The current hashmap of users is {:?}", users);

    Ok(id.hyphenated().to_string())
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

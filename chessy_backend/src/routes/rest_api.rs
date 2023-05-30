use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::AppState;

pub async fn register_user(name: String, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().await;
    log::debug!("Received username {}", &name);

    let id = Uuid::new_v4();
    log::debug!("Generated UUID ({}) for user", &id);

    users.insert(id, name);
    log::debug!("The current hashmap of users is {:?}", users);

    HttpResponse::Ok().message_body(id.hyphenated().to_string())
}

use actix_web::{http::header::ContentType, web, HttpResponse, ResponseError};

use uuid::Uuid;

use crate::AppState;

#[derive(Debug, thiserror::Error)]
pub enum RegisterErrors {
    #[error("The user name is empty!")]
    NameIsEmpty,
    #[error("Intenal server error: The users lock has been poisoned.")]
    LockPoisoned,
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

pub async fn register_user(
    name: String,
    data: web::Data<AppState>,
) -> Result<String, RegisterErrors> {
    log::debug!("Received username `{}`", &name);

    if name.is_empty() {
        return Err(RegisterErrors::NameIsEmpty);
    }

    let id = Uuid::new_v4();
    log::debug!("Generated UUID `{}` for user `{}`", &id, &name);

    let mut users = data.lock().map_err(|_| RegisterErrors::LockPoisoned)?;
    users.insert(id, name.into());
    log::debug!("The current hashmap of users is {:?}", users);

    Ok(id.hyphenated().to_string())
}

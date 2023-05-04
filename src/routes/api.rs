use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct Api {
    id: String,
}

#[derive(Serialize)]
struct User {
    id: Uuid,
    username: String,
}

pub async fn index() -> impl IntoResponse {
    let uuid = Uuid::new_v4();

    tracing::debug!("index handler: uuid={:?}", uuid);
    let user = User {
        id: uuid,
        username: "Mozart409".to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(user))
}

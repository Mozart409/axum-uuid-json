use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Health {
    health: String,
    version: String,
}

pub async fn health_check() -> impl IntoResponse {
    let version = format!(
        "{}.{}.{}{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    );
    let health = "OK";
    tracing::debug!("health handler: health={:?} version={:?}", health, version);

    (
        StatusCode::OK,
        Json(Health {
            health: health.to_string(),
            version,
        }),
    )
}

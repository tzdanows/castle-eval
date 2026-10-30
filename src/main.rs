use axum::{extract::Path, routing::get, routing::post, Json, Router};
use castle_eval::{checked_sum, slugify};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Slug {
    slug: String,
}

#[derive(Deserialize)]
struct SumRequest {
    values: Vec<i64>,
}

#[derive(Serialize)]
struct SumResponse {
    sum: i64,
}

async fn healthz() -> &'static str {
    "ok"
}

#[derive(Serialize)]
struct Version {
    version: &'static str,
}

async fn version() -> Json<Version> {
    Json(Version { version: env!("CARGO_PKG_VERSION") })
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

async fn slug(Path(text): Path<String>) -> Json<Slug> {
    Json(Slug { slug: slugify(&text) })
}

async fn sum(Json(req): Json<SumRequest>) -> Json<SumResponse> {
    Json(SumResponse { sum: checked_sum(&req.values) })
}

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/greet/{name}", get(greet))
        .route("/slug/{text}", get(slug))
        .route("/sum", post(sum))
        .route("/version", get(version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn greet_handler_returns_greeting() {
        let body = greet(Path("World".to_string())).await;
        assert_eq!(body, "Hello, World!");
    }

    #[tokio::test]
    async fn version_handler_serializes_pkg_version() {
        let body = version().await;
        assert_eq!(body.0.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(
            serde_json::to_value(&body.0).unwrap(),
            json!({ "version": "0.1.0" })
        );
    }
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind");
    axum::serve(listener, app()).await.expect("serve");
}

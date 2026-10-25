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

async fn slug(Path(text): Path<String>) -> Json<Slug> {
    Json(Slug { slug: slugify(&text) })
}

async fn sum(Json(req): Json<SumRequest>) -> Json<SumResponse> {
    Json(SumResponse { sum: checked_sum(&req.values) })
}

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/slug/{text}", get(slug))
        .route("/sum", post(sum))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind");
    axum::serve(listener, app()).await.expect("serve");
}

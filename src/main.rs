use axum::{
    extract::{rejection::JsonRejection, Path},
    http::StatusCode,
    routing::get,
    routing::post,
    Json, Router,
};
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

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
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

async fn slug(Path(text): Path<String>) -> Json<Slug> {
    Json(Slug { slug: slugify(&text) })
}

async fn sum(
    request: Result<Json<SumRequest>, JsonRejection>,
) -> Result<Json<SumResponse>, (StatusCode, Json<ErrorResponse>)> {
    let Json(req) = request.map_err(|rejection| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: rejection.body_text(),
            }),
        )
    })?;
    Ok(Json(SumResponse {
        sum: checked_sum(&req.values),
    }))
}

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/slug/{text}", get(slug))
        .route("/sum", post(sum))
        .route("/version", get(version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{header::CONTENT_TYPE, Request},
    };
    use serde_json::json;
    use tower::ServiceExt;

    async fn post_sum(body: serde_json::Value) -> (StatusCode, serde_json::Value) {
        let response = app()
            .oneshot(
                Request::post("/sum")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        assert_eq!(response.headers()[CONTENT_TYPE], "application/json");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, serde_json::from_slice(&body).unwrap())
    }

    #[tokio::test]
    async fn sum_rejects_missing_or_non_numeric_values() {
        for request in [
            json!({}),
            json!({"values": [1, "two"]}),
            json!({"values": [null]}),
        ] {
            let (status, body) = post_sum(request).await;
            assert_eq!(status, StatusCode::BAD_REQUEST);
            assert!(!body["error"].as_str().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn sum_accepts_integers_and_preserves_empty_and_saturating_sums() {
        for (values, expected) in [
            (json!([1, 2, -4]), -1),
            (json!([]), 0),
            (json!([i64::MAX, 1]), i64::MAX),
            (json!([i64::MIN, -1]), i64::MIN),
        ] {
            let (status, body) = post_sum(json!({"values": values})).await;
            assert_eq!(status, StatusCode::OK);
            assert_eq!(body, json!({"sum": expected}));
        }
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

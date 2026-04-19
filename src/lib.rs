//! Core library API for the `identity` service.

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

/// Returns the package name compiled into this crate.
pub fn package_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

/// Builds the HTTP application router.
pub fn app() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> Json<Value> {
    Json(json!({
        "service": package_name(),
        "status": "ok"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[test]
    fn package_name_matches_manifest() {
        assert_eq!(package_name(), "identity");
    }

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("request should be valid"),
            )
            .await
            .expect("response should be returned");

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should be readable");
        let payload: Value = serde_json::from_slice(&body).expect("body should be valid json");

        assert_eq!(
            payload,
            json!({
                "service": "identity",
                "status": "ok"
            })
        );
    }

    #[tokio::test]
    async fn unknown_endpoint_returns_not_found() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/unknown")
                    .body(Body::empty())
                    .expect("request should be valid"),
            )
            .await
            .expect("response should be returned");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

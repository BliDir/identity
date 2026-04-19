use axum::Router;

/// Builds the HTTP application router.
pub fn app() -> Router {
    Router::new().merge(crate::health::routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

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

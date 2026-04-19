//! Core library API for the `identity` crate.

/// Returns the package name compiled into this crate.
pub fn package_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

/// Returns the HTTP response for a health check request.
pub fn health_response() -> &'static str {
    "HTTP/1.1 200 OK\r\ncontent-type: text/plain\r\ncontent-length: 2\r\n\r\nok"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_name_matches_manifest() {
        assert_eq!(package_name(), "identity");
    }

    #[test]
    fn health_response_is_ok() {
        assert_eq!(
            health_response(),
            "HTTP/1.1 200 OK\r\ncontent-type: text/plain\r\ncontent-length: 2\r\n\r\nok"
        );
    }
}

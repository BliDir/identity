//! Core library API for the `identity` service.

pub mod health;
pub mod router;

/// Returns the package name compiled into this crate.
pub fn package_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_name_matches_manifest() {
        assert_eq!(package_name(), "identity");
    }
}

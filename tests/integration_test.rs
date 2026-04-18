#[test]
fn exposes_package_name() {
    assert_eq!(identity::package_name(), "identity");
}

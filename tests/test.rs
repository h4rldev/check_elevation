use check_elevation::is_elevated;

#[test]
fn test_is_elevated() {
    // On Windows, the test process should always be elevated when run as an administrator.
    // If we're not running as an administrator, the test will fail.
    assert_eq!(is_elevated().unwrap(), true);
}

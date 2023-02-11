use scribe::path::ScribePath;

#[test]
fn test_path_get_base() {
    let path = ScribePath::new("inbox", "test.md");
    assert_eq!(path.get_base(), Some("test.md".to_string()));
}

#[test]
fn test_path_get_category() {
    let path = ScribePath::new("inbox", "test.md");
    assert_eq!(path.get_category(), "inbox".to_string());

    let path2 = ScribePath::new("inbox/test", "test.md");
    assert_eq!(path2.get_category(), "inbox/test".to_string());
}

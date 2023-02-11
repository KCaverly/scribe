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

#[test]
fn test_path_exists() {
    let known_path = "/home/kcaverly/kb/projects/scribe/test.txt";
    let path = ScribePath::from(known_path);
    assert!(path.exists());

    let unknown_path = "/home/kcaverly/kb/projects/unknown_project/test.txt";
    let path = ScribePath::from(unknown_path);
    assert!(!path.exists());
}

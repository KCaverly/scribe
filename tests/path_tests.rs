use scribe::path::ScribePath;

pub mod common;
use common::{DIR_PATH, HIDDEN_PATH, MD_PATH, NESTED_DIR_PATH, NESTED_PATH, NOTES_DIR};

pub fn take_down_test_directory() {
    let path = ScribePath::from(&format!("{}/tmp", &*NOTES_DIR));
    path.delete();
}

#[test]
fn test_path_create_and_delete_directory() {
    take_down_test_directory();

    // Test Without Nested Directories, creating from file
    let dir_path = ScribePath::from(&DIR_PATH);
    assert!(!dir_path.exists(), "{}", dir_path.as_string(true));

    let path = ScribePath::from(&MD_PATH);
    path.create_directory();

    assert!(dir_path.exists());

    dir_path.delete();
    assert!(!dir_path.exists());

    let dir_path2 = ScribePath::from(&NESTED_DIR_PATH);
    assert!(!dir_path2.exists());

    let path2 = ScribePath::from(&NESTED_PATH);
    path2.create_directory();

    assert!(dir_path2.exists());

    dir_path2.delete();
    assert!(!dir_path2.exists());

    take_down_test_directory();
}

#[test]
fn test_path_create_and_delete_file() {
    take_down_test_directory();

    // Test Create File
    let path = ScribePath::from(&MD_PATH);
    assert!(!path.exists());
    path.create_file("This is a test file!".to_string());
    assert!(path.exists());

    // Test Delete File
    path.delete();
    assert!(!path.exists());

    take_down_test_directory();
}

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
fn test_path_is_hidden() {
    let path = ScribePath::from(&*HIDDEN_PATH);
    assert!(path.is_hidden());

    let known_path = "/home/kcaverly/kb/projects/scribe/text.txt";
    let path = ScribePath::from(known_path);
    assert!(!path.is_hidden());
}
//
// #[test]
// fn test_path_replace_category() {
//     let known_path = "/home/kcaverly/kb/projects/scribe/test.txt";
//     let mut path = ScribePath::from(known_path);
//     assert_eq!(path.get_category(), "projects/scribe");
//
//     path.replace_category("inbox/scribe");
//     assert_eq!(path.get_category(), "inbox/scribe");
// }
//
// #[test]
// fn test_path_is_markdown() {
//     let known_path = "/home/kcaverly/kb/projects/scribe/functionality.md";
//     let path = ScribePath::from(known_path);
//     assert!(path.is_markdown());
//
//     let path = "/home/kcaverly/kb/projects/scribe/test.txt";
//     let path = ScribePath::from(path);
//     assert!(!path.is_markdown());
// }

use scribe::path::ScribePath;
use std::str;

pub mod common;
use common::{DIR_PATH, HIDDEN_PATH, MD_PATH, NESTED_DIR_PATH, NESTED_PATH, NOTES_DIR, TXT_PATH};

pub fn take_down_test_directory() {
    let path = ScribePath::from(&*NOTES_DIR);
    let children = path.get_children();
    for child in children {
        if child.as_string(false).starts_with("tmp") {
            child.delete();
        }
    }
}

#[test]
fn test_path_create_and_delete_directory() {
    take_down_test_directory();

    let test_name = "tmp_create_and_delete_directory";
    let dir_path_str = &*DIR_PATH.replace("tmp", test_name);
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let nested_dir_path_str = &*NESTED_DIR_PATH.replace("tmp", test_name);
    let nested_path_str = &*NESTED_PATH.replace("tmp", test_name);

    // Test without nested directories
    let dir_path = ScribePath::from(dir_path_str);
    assert!(
        !dir_path.exists(),
        "Incorrectly Exists: {}",
        dir_path.as_string(true)
    );

    let path = ScribePath::from(md_path_str);
    path.create_directory();

    assert!(
        dir_path.exists(),
        "{}",
        format!("Exists: {}", dir_path.as_string(true))
    );

    dir_path.delete();
    assert!(!dir_path.exists());

    let dir_path2 = ScribePath::from(nested_dir_path_str);
    assert!(!dir_path2.exists());

    let path2 = ScribePath::from(nested_path_str);
    path2.create_directory();
    assert!(dir_path2.exists());

    dir_path2.delete();
    assert!(!dir_path2.exists());

    take_down_test_directory();
}

#[test]
fn test_path_create_and_delete_file() {
    take_down_test_directory();

    let test_name = "tmp_create_and_delete_file";
    let dir_path_str = &*DIR_PATH.replace("tmp", test_name);
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let nested_dir_path_str = &*NESTED_DIR_PATH.replace("tmp", test_name);
    let nested_path_str = &*NESTED_PATH.replace("tmp", test_name);

    // Test Create File
    let path = ScribePath::from(md_path_str);
    assert!(!path.exists());
    path.create_file("This is a test file!");
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
    let test_name = "tmp_path_is_hidden";
    let hidden_path_str = &*HIDDEN_PATH.replace("tmp", test_name);

    let path = ScribePath::from(hidden_path_str);
    assert!(path.is_hidden());

    let known_path = "/home/kcaverly/kb/projects/scribe/text.txt";
    let path = ScribePath::from(known_path);
    assert!(!path.is_hidden());
}

#[test]
fn test_path_replace_category() {
    let mut path = ScribePath::from(&*MD_PATH);
    assert_eq!(path.get_category(), "tmp");

    path.replace_category("inbox/scribe");
    assert_eq!(path.get_category(), "inbox/scribe");
}

#[test]
fn test_path_is_markdown() {
    let path = ScribePath::from(&*MD_PATH);
    assert!(path.is_markdown());

    let path = ScribePath::from(&*TXT_PATH);
    assert!(!path.is_markdown());
}

#[test]
fn test_path_get_data() {
    take_down_test_directory();

    let test_name = "tmp_path_get_data";
    let md_path_str = &*MD_PATH.replace("tmp", test_name);

    let path = ScribePath::from(md_path_str);
    let test_data = "This is a test file.";
    path.create_file(test_data);

    let data = path.get_data();
    assert!(data.is_some());
    assert_eq!(test_data, data.unwrap());

    take_down_test_directory();
}

#[test]
fn test_path_get_children() {
    take_down_test_directory();

    let path = ScribePath::from(&*NOTES_DIR);
    let children = path.get_children();

    take_down_test_directory();
}

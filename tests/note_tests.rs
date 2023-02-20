use scribe::{
    note::Note,
    path::ScribePath,
    template::{ScribeTemplate, ScribeTemplateLibrary},
};
use std::collections::HashMap;

pub mod common;
use common::{MD_PATH, NOTES_DIR};

pub fn take_down_test_directory() {
    let path = ScribePath::from(&format!("{}/tmp", &*NOTES_DIR));
    path.delete();
}

#[test]
fn test_note_from_template() {
    take_down_test_directory();

    let path = ScribePath::from(&*MD_PATH.replace("tmp", "tmp_note_from_template"));
    let library = ScribeTemplateLibrary::new();
    let template = library.get_template("basic").unwrap();

    let mut params = HashMap::new();
    params.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    params.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    params.insert("TITLE".to_string(), "This is a test title".to_string());
    let _note = Note::from_template(path, template, params);

    take_down_test_directory();
}

#[test]
fn test_note_from_path() {
    take_down_test_directory();

    // Create File
    let path = ScribePath::from(&*MD_PATH.replace("tmp", "tmp_note_from_template"));
    let library = ScribeTemplateLibrary::new();
    let template = library.get_template("basic").unwrap();

    let mut params = HashMap::new();
    params.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    params.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    params.insert("TITLE".to_string(), "This is a test title".to_string());
    let _note = Note::from_template(path, template, params);

    // Test that you can read back in this file
    let path = ScribePath::from(&*MD_PATH.replace("tmp", "tmp_note_from_template"));
    let _note = Note::from_path(path);

    take_down_test_directory();
}

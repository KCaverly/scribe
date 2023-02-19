use chrono::{Local, NaiveDateTime, TimeZone};
use scribe::note::Note;
use scribe::parsers::date::Date;
use scribe::parsers::embedded_links::EmbeddedLinks;
use scribe::parsers::internal_links::InternalLinks;
use scribe::parsers::parser::Parser;
use scribe::parsers::tags::Tags;
use scribe::parsers::title::Title;
use scribe::parsers::web_links::WebLinks;
use scribe::path::ScribePath;
use scribe::template::ScribeTemplate;
use std::collections::{HashMap, HashSet};

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
fn test_parser_get_matches() {
    take_down_test_directory();

    let test_name = "tmp_parser_get_matches";
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let md_path = ScribePath::from(md_path_str);

    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert("TITLE".to_string(), "This is a test title".to_string());

    let note = Note::from_template(md_path, template, values);

    let md_path = ScribePath::from(md_path_str);
    let parser = Parser::new("(date:)".to_string());
    let data = md_path.get_data().unwrap();
    let matches = parser.get_matches(&data);

    take_down_test_directory();
}

#[test]
fn test_parser_date() {
    take_down_test_directory();

    let test_name = "tmp_parser_date";
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let md_path = ScribePath::from(md_path_str);

    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert("TITLE".to_string(), "This is a test title".to_string());

    let test_date =
        NaiveDateTime::parse_from_str("2023-01-30 09:56 PM", "%Y-%m-%d %I:%M %p").unwrap();
    let test_date = Local.from_local_datetime(&test_date).unwrap();

    let note = Note::from_template(md_path, template, values);

    let md_path = ScribePath::from(md_path_str);
    let data = md_path.get_data().unwrap();
    let date = Date::parse(&data);

    assert_eq!(date, Some(test_date));

    take_down_test_directory();
}

#[test]
fn test_parser_title() {
    take_down_test_directory();

    let test_name = "tmp_parser_title";
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let md_path = ScribePath::from(md_path_str);

    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert("TITLE".to_string(), "This is a test title".to_string());

    let note = Note::from_template(md_path, template, values);

    let test_title = "This is a test title".to_string();

    let md_path = ScribePath::from(md_path_str);
    let data = md_path.get_data().unwrap();
    let title = Title::parse(&data);

    assert_eq!(title, Some(test_title));

    take_down_test_directory();
}

#[test]
fn test_parser_tags() {
    take_down_test_directory();

    let test_name = "tmp_parser_tags";
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let md_path = ScribePath::from(md_path_str);

    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert("TITLE".to_string(), "This is a test title".to_string());

    let note = Note::from_template(md_path, template, values);

    let md_path = ScribePath::from(md_path_str);
    let data = md_path.get_data().unwrap();
    let tags = Tags::parse(&data);

    let mut test_tags = HashSet::<String>::new();
    test_tags.insert("tag1".to_string());
    test_tags.insert("tag2".to_string());

    assert_eq!(tags, Some(test_tags));

    take_down_test_directory();
}

#[test]
fn test_parser_embedded_links() {
    take_down_test_directory();

    let test_name = "tmp_parser_embedded_links";
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let md_path = ScribePath::from(md_path_str);

    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert(
        "TITLE".to_string(),
        "This [[test/link title | Title]]".to_string(),
    );

    let note = Note::from_template(md_path, template, values);

    let md_path = ScribePath::from(md_path_str);
    let data = md_path.get_data().unwrap();
    let embedded_links = EmbeddedLinks::parse(&data);

    let mut test_links = HashSet::<String>::new();
    test_links.insert("test/link title".to_string());

    assert_eq!(Some(test_links), embedded_links);

    take_down_test_directory();
}

#[test]
fn test_parser_web_links() {
    take_down_test_directory();

    let test_name = "tmp_parser_web_links";
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let md_path = ScribePath::from(md_path_str);

    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert(
        "TITLE".to_string(),
        "This https://www.google.com)".to_string(),
    );

    let note = Note::from_template(md_path, template, values);

    let md_path = ScribePath::from(md_path_str);
    let data = md_path.get_data().unwrap();
    let web_links = WebLinks::parse(&data);

    let mut test_links = HashSet::<String>::new();
    test_links.insert("https://www.google.com".to_string());

    assert_eq!(Some(test_links), web_links);

    take_down_test_directory();
}

#[test]
fn test_parser_internal_links() {
    take_down_test_directory();

    let test_name = "tmp_parser_internal_links";
    let md_path_str = &*MD_PATH.replace("tmp", test_name);
    let md_path = ScribePath::from(md_path_str);

    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert(
        "TITLE".to_string(),
        "This [internal_data](/home/kcaverly/test/)".to_string(),
    );

    let note = Note::from_template(md_path, template, values);

    let md_path = ScribePath::from(md_path_str);
    let data = md_path.get_data().unwrap();
    let links = InternalLinks::parse(&data);

    let mut test_links = HashSet::<String>::new();
    test_links.insert("/home/kcaverly/test/".to_string());

    assert_eq!(Some(test_links), links);

    take_down_test_directory();
}

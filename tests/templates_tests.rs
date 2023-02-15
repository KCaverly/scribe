use scribe::template::ScribeTemplate;
use std::collections::HashMap;

#[test]
fn test_template_load_template() {
    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let _template = ScribeTemplate::load(path);
}

#[test]
fn test_template_get_keys() {
    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);
    let keys = template.get_keys();
    assert!(keys.contains(&"TAGS".to_string()));
    assert!(keys.contains(&"TITLE".to_string()));
    assert!(keys.contains(&"DATE".to_string()));
}

#[test]
fn test_template_fill() {
    let path = "/home/kcaverly/personal/scribe/src/templates/basic.md";
    let template = ScribeTemplate::load(path);

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert("TITLE".to_string(), "This is a test title".to_string());

    let filled = template.fill(&values);
}

mod cli;
mod link;
mod note;
mod path;
mod scribe;
mod template;

use std::collections::HashMap;
use std::env;
use std::process::exit;
use template::ScribeTemplate;

use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref NOTES_DIR: String = env::var("NOTES_DIR").unwrap();
}

fn main() {
    let template = ScribeTemplate::load("/home/kcaverly/personal/scribe/src/templates/basic.md");

    let mut values = HashMap::new();
    values.insert("DATE".to_string(), "2023-01-30 09:56 PM".to_string());
    values.insert("TAGS".to_string(), r#""tag1","tag2""#.to_string());
    values.insert("TITLE".to_string(), "this is a test title".to_string());

    let filled_template = template.fill(&values);
    println!("{}", filled_template);

    if env::var("NOTES_DIR").is_err() {
        println!("Please set NOTES_DIR before continuing.");
        exit(0)
    }

    cli::run_cli();
}

mod cli;
mod link;
mod note;
mod path;
mod scribe;
mod template;

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

    let keys = template.get_keys();

    for key in keys {
        println!("{}", key);
    }

    if env::var("NOTES_DIR").is_err() {
        println!("Please set NOTES_DIR before continuing.");
        exit(0)
    }

    cli::run_cli();
}

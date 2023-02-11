mod cli;
mod link;
mod note;
mod path;
mod scribe;
mod template;

use std::collections::HashMap;
use std::env;
use std::process::exit;
use scribe::Scribe;
use template::ScribeTemplate;

use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref NOTES_DIR: String = env::var("NOTES_DIR").unwrap();
}

fn main() {

    let tags = Scribe::list_tags();
    for tag in tags {
        println!("{}", tag);
    }

    if env::var("NOTES_DIR").is_err() {
        println!("Please set NOTES_DIR before continuing.");
        exit(0)
    }

    // cli::run_cli();
}

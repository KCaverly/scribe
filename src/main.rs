mod note;
mod path;
mod cli;
mod scribe;
mod link;

use std::env;
use std::process::exit;

use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref NOTES_DIR: String = env::var("NOTES_DIR").unwrap();
}

fn main() {
    if env::var("NOTES_DIR").is_err() {
        println!("Please set NOTES_DIR before continuing.");
        exit(0)
    }

    cli::run_cli();

}

pub mod config;
pub mod index;
pub mod note;
pub mod parsers;
pub mod path;
pub mod sync;
pub mod template;

use lazy_static::lazy_static;
use std::env;

lazy_static! {
    #[derive(Debug)]
    static ref NOTES_DIR: String = env::var("NOTES_DIR").unwrap();
}

use std::fmt;

#[derive(Debug)]
pub enum ScribeError {
    MissingParams,
}

impl std::error::Error for ScribeError {}

impl fmt::Display for ScribeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScribeError::MissingParams => write!(f, "HTTP Error"),
        }
    }
}

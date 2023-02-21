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

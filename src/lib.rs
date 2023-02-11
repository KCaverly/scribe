pub mod template;
pub mod path;

use lazy_static::lazy_static;
use std::env;

lazy_static! {
    #[derive(Debug)]
    static ref NOTES_DIR: String = env::var("NOTES_DIR").unwrap();
}

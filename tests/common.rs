use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref NOTES_DIR: String = format!("{}", env::var("NOTES_DIR").unwrap());
    pub static ref MD_PATH: String = format!("{}/tmp/test.md", &*NOTES_DIR);
    pub static ref DIR_PATH: String = format!("{}/tmp", &*NOTES_DIR);
    pub static ref NESTED_PATH: String = format!("{}/tmp/test/test.md", &*NOTES_DIR);
    pub static ref NESTED_DIR_PATH: String = format!("{}/tmp/test", &*NOTES_DIR);
    pub static ref HIDDEN_PATH: String = format!("{}/temp/test/.hidden.md", &*NOTES_DIR);
}


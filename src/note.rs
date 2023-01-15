// use chrono::DateTime;
use chrono;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct Note {
    id: u64,
    category: String,
    group: String,
    //path: PathBuf,
    title: String,
    author: String,
    // date: DateTime,
    tags: Vec<String>,
}

impl Note {
    fn get_path(category: &str, title: &str) -> PathBuf {
        let path_str = format!("{0}/{1}/{2}.md", "/home/kcaverly/kb", category, title);

        return PathBuf::from(path_str);
    }

    fn init(category: &str, title: &str) {
        let mut p = Self::get_path(category, title);
        println!("{}", p.exists());

        if !p.exists() {
            // Create File
            println!("{}", p.as_path().display().to_string());
            let mut f =
                File::create(p.as_path().display().to_string()).expect("Unable to create file");

            let init_data =
                format!("\n<!--- ID: 0 --->\n<!--- CATEGORY: {category}--->\n\n# {title}\n");

            f.write_all(init_data.as_bytes());
            // fs::write(p.as_path().display().to_string(), "test").expect("UNABLE");
        }
    }

    pub fn new(category: &str, title: &str) {
        Self::init(category, title);
    }
}

use chrono::{self, DateTime, Local};
use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;
use std::error::Error;
use std::fs::{self, rename, File};
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn search_notes(search_string: String) -> Result<(Vec<(String, String)>), Box<dyn Error>> {
    let mut matches: Vec<(String, String)> = vec![];
    for entry in WalkDir::new("/home/kcaverly/kb")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            let file_data =
                fs::read(entry.path().display().to_string()).expect("Cannot open file!");
            let matcher = RegexMatcher::new(&search_string)?;
            Searcher::new().search_slice(
                &matcher,
                &file_data,
                UTF8(|lnum, line| {
                    let mymatch = matcher.find(line.as_bytes())?.unwrap();
                    matches.push((
                        entry.path().display().to_string(),
                        line[mymatch].to_string(),
                    ));
                    Ok(true)
                }),
            )?;
        }
    }
    Ok(matches)
}

pub struct Note {
    category: String,
    name: String,
    tags: Vec<String>,
    date: DateTime<Local>,
}

pub trait NoteManager {
    fn new(
        category: Option<String>,
        name: String,
        tags: Option<Vec<String>>,
        date: Option<DateTime<Local>>,
    ) -> Self;

    fn transfer(path: String, category: String);

    fn path(&self) -> PathBuf;

    fn init(&self);
}

impl NoteManager for Note {
    fn new(
        category: Option<String>,
        name: String,
        tags: Option<Vec<String>>,
        date: Option<DateTime<Local>>,
    ) -> Note {
        let c: String;
        if category.is_some() {
            c = category.unwrap();
        } else {
            c = "inbox".to_string();
        }

        let t: Vec<String>;
        if tags.is_some() {
            t = tags.unwrap();
        } else {
            t = Vec::new();
        }

        let d: DateTime<Local>;
        if date.is_some() {
            d = date.unwrap();
        } else {
            d = chrono::Local::now();
        }

        return Note {
            name: name,
            category: c,
            date: d,
            tags: t,
        };
    }

    fn transfer(path: String, category: String) {
        let path_clone = path.clone();
        let file_path = PathBuf::from(path);
        if file_path.is_dir() {
            let current_category = path_clone.replace("/home/kcaverly/kb", "");
            let name = current_category.split("/").last().unwrap();

            let new_path =
                path_clone.replace(&current_category, &format!("/{}/{}", &category, name));

            rename(path_clone, new_path);
        };
    }

    fn path(&self) -> PathBuf {
        let title = self.name.to_lowercase().replace(" ", "_");
        let path = format!("{0}/{1}/{2}.md", "/home/kcaverly/kb", self.category, title);
        return PathBuf::from(path);
    }

    fn init(&self) {
        println!("Initializing note: {}", self.name);

        let mut p = self.path();

        if !p.exists() {
            println!("File Does not Exist!");

            // Create directory if missing
            if !p.parent().unwrap().exists() {
                _ = fs::create_dir(p.parent().unwrap());
            }

            // Create File
            let mut f =
                File::create(p.as_path().display().to_string()).expect("Unable to create file");

            let date = self.date.format("%Y-%m-%d %I:%M %p");
            let mut init_str = vec![
                format!("# {0}\n", self.name),
                format!("\n**Date:** {date}  "),
                format!("\n**Tags:** "),
            ];

            for tag in &self.tags {
                init_str.push(format!("#{tag} "));
            }

            let init_data = init_str.join("");
            _ = f.write_all(init_data.trim().as_bytes());
        }
    }
}

// fn init(category: &str, title: &str, tags: Vec<&str>) {
//     let mut p = Self::get_path(category, title);
//     println!("{}", p.exists());
//
//     if !p.exists() {
//         // Create Directory if missing
//         if !p.parent().unwrap().exists() {
//             fs::create_dir(p.parent().unwrap());
//         }
//
//         // Create File
//         let mut f =
//             File::create(p.as_path().display().to_string()).expect("Unable to create file");
//
//         let date =
//         let dt_str = date.format("%Y-%m-%d %I:%M %p");
//
//         let mut init_str = vec![
//             format!("# {title}\n"),
//             format!("\n**Date:** {dt_str}  "),
//             format!("\n**Tags:**"),
//         ];
//
//         for tag in tags {
//             init_str.push(format!("#{tag} "));
//         }
//
//         let init_data = init_str.join("");
//
//         f.write_all(init_data.as_bytes());
//     }
// }

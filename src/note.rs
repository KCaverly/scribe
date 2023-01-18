use chrono::{self, DateTime, Local, NaiveDateTime, TimeZone};
use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;
use std::error::Error;
use std::fs::{self, rename, File};
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::process::{Command,Stdio};
use std::env;

#[derive(Debug)]
pub struct Note {
    category: String,
    name: String,
    tags: Vec<String>,
    date: DateTime<Local>,
}

impl Note {
    pub fn new(
        category: Option<String>,
        name: String,
        tags: Option<Vec<String>>,
        date: Option<DateTime<Local>>,
    ) -> Self {
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

        return Self {
            name: name,
            category: c,
            date: d,
            tags: t,
        };
    }

    pub fn from_path(path: String) -> Self {
        let path_clone = path.clone();
        let data = fs::read(path).expect("Cannot open file!");

        // Get Tags
        let tag_line = Self::search_data(&data, "Tags:.+".to_string()).unwrap();
        let tags = tag_line.last().unwrap().replace("Tags:**", "");
        let tag_vec = tags
            .trim()
            .split(" #")
            .map(|s| s.trim_start_matches("#").to_string())
            .collect();

        // Get Date
        let date_line = Self::search_data(&data, "Date:.+".to_string()).unwrap();
        let date_str = date_line.last().unwrap().replace("Date:** ", "");
        let naive = NaiveDateTime::parse_from_str(&date_str.trim(), "%Y-%m-%d %I:%M %p").unwrap();
        let date: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();

        // Get title
        let title_line = Self::search_data(&data, "^# .+".to_string()).unwrap();
        let title = title_line.last().unwrap().replace("# ", "");

        // Get Category
        let file_name = Self::normalize_title(&title);
        let category = path_clone
            .replace(&NoteManager::get_notes_directory(), "")
            .replace(&file_name, "")
            .replace(".md", "");

        let n = Note::new(
            Some(
                category
                    .trim_end_matches("/")
                    .trim_start_matches("/")
                    .to_string(),
            ),
            title.trim().to_string(),
            Some(tag_vec),
            Some(date),
        );

        return n;
    }

    pub fn transfer(path: String, category: String) {
        let path_clone = path.clone();
        let file_path = PathBuf::from(path);
        if file_path.is_dir() {
            let current_category = path_clone.replace(&NoteManager::get_notes_directory(), "");
            let name = current_category.split("/").last().unwrap();
            let new_path =
                path_clone.replace(&current_category, &format!("/{}/{}", &category, name));
            rename(path_clone, new_path);
        };
    }

    pub fn search_data(data: &[u8], search_string: String) -> Result<Vec<String>, Box<dyn Error>> {
        let mut matches: Vec<String> = vec![];
        let matcher = RegexMatcher::new(&search_string)?;
        Searcher::new().search_slice(
            &matcher,
            data,
            UTF8(|lnum, line| {
                let mymatch = matcher.find(line.as_bytes())?.unwrap();
                matches.push(line[mymatch].to_string());
                Ok(true)
            }),
        )?;
        return Ok(matches);
    }

    fn normalize_title(title: &str) -> String {
        return title.to_lowercase().replace(" ", "_");
    }

    pub fn path(&self) -> PathBuf {
        let title = Self::normalize_title(&self.name);
        let path = format!("{0}/{1}/{2}.md", &NoteManager::get_notes_directory(), self.category, title);
        return PathBuf::from(path);
    }

    pub fn init(&self) {

        let p = self.path();

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

pub struct NoteManager {}

impl NoteManager {

    pub fn get_notes_directory() -> String {
        if env::var("NOTES_DIR").is_ok() {
            let notes_dir = env::var("NOTES_DIR").unwrap();
            return notes_dir;
        } else {
            panic!("Please set NOTES_DIR in your environment variables!");
        }
    }

    pub fn search_notes(search_string: String) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let mut matches: Vec<(String, String)> = vec![];
        let notes_dir = Self::get_notes_directory();
        for entry in WalkDir::new(notes_dir)
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

    pub fn save_notes(commit_message: String) {
        let notes_dir = Self::get_notes_directory();

        // Add all notes to directory
        for entry in WalkDir::new(&notes_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                let add = Command::new("git").current_dir(&notes_dir).args(vec!["add", &entry.path().display().to_string()]).status().unwrap();
            }
        }

        // Commit notes to directory
        let commit = Command::new("git").stdout(Stdio::null()).current_dir(&notes_dir).args(vec!["commit", "-m", &commit_message]).status().unwrap();

        if commit.success() {
            let push = Command::new("git").stdout(Stdio::null()).stderr(Stdio::null()).current_dir(&notes_dir).args(vec!["push"]).status().unwrap();
            println!("Notes Saved!");
        } else {
            println!("No Change to Save!");
        }
    }
}

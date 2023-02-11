use crate::path::ScribePath;
use crate::scribe::Scribe;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::Regex;

use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::str;

pub struct Note {
    title: String,
    category: String,
    // TODO: Convert this to HashSet<String>
    // We shouldnt have duplicates in this list
    pub tags: Vec<String>,
    date: DateTime<Local>,
    path: ScribePath,
}

impl Note {
    // Constructors
    pub fn new(title: String, category: Option<String>, tags: Option<Vec<String>>) -> Self {
        // Default Category is Inbox
        let c: String;
        if category.is_some() {
            c = category.unwrap();
        } else {
            c = "inbox".to_string();
        }

        // Default Tags are <>
        let t: Vec<String>;
        if tags.is_some() {
            t = tags.unwrap();
        } else {
            t = Vec::new();
        }

        // Default Date to Now
        let date = Local::now();

        // Create Default Path
        let path = ScribePath::new(
            &c,
            &format!("{}.md", title.to_lowercase().replace(" ", "_")),
        );

        return Self {
            title: title,
            category: c,
            date: date,
            tags: t,
            path: path,
        };
    }
    pub fn from_path(path: &str) -> Self {
        // Read in Data
        let data = fs::read(path).expect("Cannot open file!");
        let scribe_path = ScribePath::from(path);

        let tags = Self::parse_tags(&data);
        let date = Self::parse_date(&data);
        let title = Self::parse_title(&data);

        return Self {
            title: title,
            category: scribe_path.category.to_string(),
            date: date,
            tags: tags,
            path: scribe_path,
        };
    }

    // Private Methods
    fn get_matches<'a>(data: &'a [u8], capture_string: &'a str) -> Option<HashSet<&'a str>> {
        let re: Regex = Regex::new(capture_string).unwrap();

        let data_ = str::from_utf8(data);
        if data_.is_err() {
            return None;
        }
        let caps = re.captures_iter(str::from_utf8(data).unwrap());
        let res: HashSet<&str> = caps
            .map(|m| m.get(1))
            .map(|x| x.expect("Failed").as_str())
            .collect();
        return Some(res);
    }

    fn get_file_name(&self) -> String {
        if self.path.base.is_some() {
            return self.path.base.as_ref().unwrap().to_string();
        } else {
            panic!("This has no file name!");
        }
    }

    pub fn parse_tags(data: &[u8]) -> Vec<String> {
        // TODO: Add Frontmatter matches
        let hashtag_matches = Self::get_matches(data, "\\#([A-Za-z0-9-]+)");

        if hashtag_matches.is_none() {
            return vec![];
        }

        let tags: Vec<String> = Vec::from_iter(hashtag_matches.unwrap())
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        return tags;
    }

    pub fn parse_title(data: &[u8]) -> String {
        return "".to_string();
    }

    pub fn parse_date(data: &[u8]) -> DateTime<Local> {
        let date_line = Some("2020-01-01 10:02 PM");
        let date_str = date_line.unwrap().replace("Date:** ", "");
        let naive = NaiveDateTime::parse_from_str(&date_str.trim(), "%Y-%m-%d %I:%M %p").unwrap();
        return Local.from_local_datetime(&naive).unwrap();
    }

    // Public Methods
    pub fn edit(&self) {
        unimplemented!("Edit not yet implemented!");
    }

    pub fn init(&self) {
        let p = self.path.as_pathbuf();
        if !p.exists() {
            // Create directory if missing
            if !p.parent().unwrap().exists() {
                _ = fs::create_dir(p.parent().unwrap());
            }

            // Create File
            let mut file = File::create(self.path.as_string(true)).expect("Unable to create file!");
            let date = self.date.format("%Y-%m-%d %I:%M %p");
            let mut init_str = vec![
                format!("# {0}\n", self.title),
                format!("\n**Date:** {date}  "),
                format!("\n**Tags:** "),
            ];

            for tag in &self.tags {
                init_str.push(format!("#{tag} "));
            }

            let init_data = init_str.join("");
            _ = file.write_all(init_data.trim().as_bytes());
        }
    }
}

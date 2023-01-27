use crate::path::ScribePath;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
// For Searching
use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;

pub struct Note {
    title: String,
    category: String,
    tags: Vec<String>,
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
    fn search_data(data: &[u8], search_string: String) -> Result<Vec<String>, Box<dyn Error>> {
        // Search
        let mut matches: Vec<String> = vec![];
        let matcher = RegexMatcher::new(&search_string)?;
        Searcher::new().search_slice(
            &matcher,
            &data,
            UTF8(|lnum, line| {
                let mymatch = matcher.find(line.as_bytes())?.unwrap();
                matches.push(line[mymatch].to_string());
                Ok(true)
            }),
        )?;
        return Ok(matches);
    }

    fn get_file_name(&self) -> String {
        if self.path.base.is_some() {
            return self.path.base.as_ref().unwrap().to_string();
        } else {
            panic!("This has no file name!");
        }
    }

    fn parse_tags(data: &[u8]) -> Vec<String> {
        let tag_line = Self::search_data(data, "Tags:.+".to_string()).unwrap();
        let tags = tag_line.last().unwrap().replace("Tags:**", "");
        let tag_vec = tags
            .trim()
            .split(" #")
            .map(|s| s.trim_start_matches("#").to_string())
            .collect();
        return tag_vec;
    }

    fn parse_date(data: &[u8]) -> DateTime<Local> {
        let date_line = Self::search_data(data, "Date:.+".to_string()).unwrap();
        let date_str = date_line.last().unwrap().replace("Date:** ", "");
        let naive = NaiveDateTime::parse_from_str(&date_str.trim(), "%Y-%m-%d %I:%M %p").unwrap();
        return Local.from_local_datetime(&naive).unwrap();
    }

    fn parse_title(data: &[u8]) -> String {
        let title_line = Self::search_data(data, "^# .+".to_string()).unwrap();
        return title_line.last().unwrap().replace("# ", "");
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

    pub fn search_match(&self, match_string: String) -> Result<Vec<String>, Box<dyn Error>> {
        unimplemented!("Regex Matching not yet implemented!");
    }

    pub fn search(&self, search_string: String) -> Result<Vec<String>, Box<dyn Error>> {
        // Get Data
        let path = self.path.as_string(true);
        let data = fs::read(path).expect("Cannot open file!");

        return Self::search_data(&data, search_string);
    }
}

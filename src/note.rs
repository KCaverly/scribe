use crate::path::PJPath;
use crate::NOTES_DIR;
use casual;
use chrono::{self, DateTime, Local, NaiveDateTime, TimeZone};
use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;
use skim::prelude::*;
use std::env;
use std::error::Error;
use std::fs::{self, rename, File};
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Note {
    category: String,
    name: String,
    tags: Vec<String>,
    date: DateTime<Local>,
}

impl Note {
    pub fn edit(&self) {
        let editor_var: String;
        if env::var("EDITOR").is_ok() {
            editor_var = env::var("EDITOR").unwrap();
        } else {
            println!("PLEASE SET $EDITOR TO LAUNCH EDITOR");
            exit(0);
        }

        let mut editor = Command::new(editor_var)
            .arg(self.path().as_path().display().to_string())
            .current_dir(&*NOTES_DIR)
            .spawn()
            .unwrap();

        let _res = editor.wait().unwrap();
    }

    pub fn parse_tags(tags: Option<String>) -> Option<Vec<String>> {
        let tags_vec: Option<Vec<String>>;
        if tags.is_some() {
            let t = tags.unwrap();
            let vec: Vec<&str> = t.split(",").collect();
            let mut tv = Vec::<String>::new();
            for v in vec {
                tv.push(v.trim().to_string());
            }
            tags_vec = Some(tv);
        } else {
            tags_vec = None;
        };

        return tags_vec;
    }

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

    pub fn from_path(path: &str) -> Self {
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
            .replace(&*NOTES_DIR, "")
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

    pub fn filepath(&self) -> String {
        return format!("{}.md", self.name.to_lowercase().replace(" ", "_"));
    }

    fn normalize_title(title: &str) -> String {
        return title.to_lowercase().replace(" ", "_");
    }

    pub fn path(&self) -> PathBuf {
        let title = Self::normalize_title(&self.name);
        let path = format!("{0}/{1}/{2}.md", &*NOTES_DIR, self.category, title);
        return PathBuf::from(path);
    }

    pub fn init(&self) {
        let p = self.path();

        if !p.exists() {
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
    pub fn interactive_create() -> Note {
        println!("Creating new note...\n");

        // Get Name of Note
        let name: String = casual::prompt("Name of the Note:    ").get();

        // Get Category of Note
        let category: String = casual::prompt("Category of Note:    ").get();

        // Get Tags Associated With Note
        let tags: String = casual::prompt("Tags for Note:       ")
            .default("".to_string())
            .get();

        let tags_vec = Note::parse_tags(Some(tags));

        println!("{}", name);

        let note = Note::new(Some(category), name, tags_vec, None);
        note.init();

        return note;
    }

    pub fn interactive_transfer() {
        println!("Transfering Notes...");
        let paths = Self::get_paths();

        // Get List of Items to Select On
        let mut items: Vec<String> = vec![];
        // Get Categories to Transfer To
        let mut categories: Vec<String> = vec![];

        for path in paths {
            items.push(path.path);

            let cat = path.category;
            if !categories.contains(&cat) {
                categories.push(cat);
            }
        }

        let selected_items = Self::finder(items, "Item to Transfer:  ", true);
        let items_clone = selected_items.clone();
        let selected_category = Self::finder(categories, "Target Category:  ", false);

        let target_category = selected_category.first().unwrap();

        println!("Transfering...\n");
        for item in selected_items {
            println!("{}", item.text());
        }

        println!("\nTO: {}\n", target_category.text());

        if casual::confirm("Confirm?") {
            for item in items_clone {
                Self::transfer(&item.text(), &target_category.text());
            }
        }
    }

    pub fn finder(
        search_options: Vec<String>,
        prompt: &str,
        multi: bool,
    ) -> Vec<Arc<dyn SkimItem>> {
        let search_string: String = search_options.join("\n");
        let options = SkimOptionsBuilder::default()
            .prompt(Some(prompt))
            .multi(multi)
            .build()
            .unwrap();

        let item_reader = SkimItemReader::default();
        let items = item_reader.of_bufread(Cursor::new(search_string));
        let selected_items = Skim::run_with(&options, Some(items))
            .filter(|out| !out.is_abort)
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());

        // If no item selected, exit silently.
        if selected_items.len() == 0 {
            exit(0)
        }

        return selected_items;
    }

    pub fn get_paths() -> Vec<PJPath> {
        let mut paths: Vec<PJPath> = vec![];
        for entry in WalkDir::new(&*NOTES_DIR).into_iter().filter_map(|e| e.ok()) {
            if !entry.path().display().to_string().contains(".git") {
                let path = PJPath::parse(&entry.path().display().to_string());
                paths.push(path);
            }
        }
        return paths;
    }

    // TODO: Accomodate for relative paths here
    pub fn transfer(path: &str, category: &str) -> std::io::Result<()> {
        // Transfer File Over
        let old_path = PJPath::parse(path);
        let mut new_path = PJPath::parse(path);
        new_path.replace_category(category);

        if !old_path.exists() {
            return Ok(());
        }

        rename(&old_path.path, &new_path.path)?;

        // Transfer Links
        let results = Self::search_notes(format!("[A-Z0-9a-z/]+?{}", old_path.relative_path()));
        if results.is_ok() {
            for res in results.unwrap() {
                let og_data = fs::read_to_string(&res.0).unwrap();
                let new_data = og_data.replace(&res.1, &new_path.relative_path());

                let mut f = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&res.0)?;
                f.write_all(new_data.as_bytes())?;
                f.flush()?;
            }
        }
        Ok(())
    }

    pub fn search_notes(search_string: String) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let mut matches: Vec<(String, String)> = vec![];
        for entry in WalkDir::new(&*NOTES_DIR).into_iter().filter_map(|e| e.ok()) {
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

    pub fn pull() {
        let pull = Command::new("git")
            .current_dir(&*NOTES_DIR)
            .args(vec!["pull"])
            .status()
            .unwrap();
    }

    pub fn save(commit_message: &str) {
        // Add all notes to directory
        for entry in WalkDir::new(&*NOTES_DIR).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                let add = Command::new("git")
                    .current_dir(&*NOTES_DIR)
                    .args(vec!["add", &entry.path().display().to_string()])
                    .status()
                    .unwrap();
            }
        }

        // Commit notes to directory
        let commit = Command::new("git")
            .stdout(Stdio::null())
            .current_dir(&*NOTES_DIR)
            .args(vec!["commit", "-m", &commit_message])
            .status()
            .unwrap();

        if commit.success() {
            let push = Command::new("git")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .current_dir(&*NOTES_DIR)
                .args(vec!["push"])
                .status()
                .unwrap();
            println!("Notes Saved!");
        } else {
            println!("No Change to Save!");
        }
    }
}

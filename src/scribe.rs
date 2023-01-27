use crate::note::Note;
use crate::path::ScribePath;
use crate::NOTES_DIR;
use casual;
use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;
use skim::prelude::*;
use std::error::Error;
use std::fs::{self, rename};
use std::io::Cursor;
use std::io::Write;
use std::process::{exit, Command, Stdio};
use walkdir::WalkDir;

pub struct Scribe {}

impl Scribe {
    // Public Functions
    pub fn create(title: String, category: Option<String>, tags: Option<Vec<String>>) -> Note {
        let note = Note::new(title, category, tags);
        note.init();

        return note;
    }

    pub fn transfer(path: &str, category: &str) -> std::io::Result<()> {
        // Transfer File Over
        let old_path = ScribePath::from(path);
        let mut new_path = ScribePath::from(path);
        new_path.replace_category(category);

        if !old_path.exists() {
            // TODO: Update error handling if path you are trying to move doesnt exist
            return Ok(());
        }

        rename(&old_path.as_string(true), &new_path.as_string(true));

        // Transfer Links
        let results = Self::search(&format!("[A-Za-z0-9/]+?{}", old_path.as_string(false)));
        if results.is_ok() {
            for res in results.unwrap() {
                // TODO: Move this into a note::Note public function
                let og_data = fs::read_to_string(&res.0).unwrap();
                let new_data = og_data.replace(&res.1, &new_path.as_string(false));

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

        let tags_vec = Self::parse_tags_string(Some(tags));

        println!("{}", name);

        let note = Note::new(name, Some(category), tags_vec);
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

        let selected_items = Self::fuzzy_finder(items, "Item to Transfer:  ", true);
        let items_clone = selected_items.clone();
        let selected_category = Self::fuzzy_finder(categories, "Target Category:  ", false);

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

    pub fn search(search_string: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
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

    pub fn parse_tags_string(tags: Option<String>) -> Option<Vec<String>> {
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

    pub fn fuzzy_finder(
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

    pub fn pull() -> bool {
        let pull_cmd = Command::new("git")
            .arg("pull")
            .current_dir(&*NOTES_DIR)
            .stdout(Stdio::null())
            .status()
            .unwrap();

        return pull_cmd.success();
    }

    pub fn save(commit_message: &str) -> bool {
        let add_cmd = Command::new("git")
            .args(vec!["add", "."])
            .current_dir(&*NOTES_DIR)
            .stdout(Stdio::null())
            .status()
            .unwrap();

        if add_cmd.success() {
            let command_cmd = Command::new("git")
                .args(vec!["commit", "-m", commit_message])
                .current_dir(&*NOTES_DIR)
                .stdout(Stdio::null())
                .status()
                .unwrap();

            if command_cmd.success() {
                let push_cmd = Command::new("git")
                    .arg("push")
                    .current_dir(&*NOTES_DIR)
                    .stdout(Stdio::null())
                    .status()
                    .unwrap();

                return push_cmd.success();
            } else {
                return false;
            }
        }
        return false;
    }

    pub fn sync(commit_message: Option<String>) -> bool {
        let git_pull = Self::pull();
        if git_pull {
            let git_save: bool;
            if commit_message.is_some() {
                git_save = Self::save(&commit_message.unwrap());
            } else {
                let msg = "(scribe) - updated notes";
                git_save = Self::save(&msg);
            }

            return git_save;
        }

        return false;
    }

    fn get_paths() -> Vec<ScribePath> {
        let mut paths: Vec<ScribePath> = vec![];
        for entry in WalkDir::new(&*NOTES_DIR).into_iter().filter_map(|e| e.ok()) {
            if !entry.path().display().to_string().contains(".git") {
                let path = ScribePath::from(&entry.path().display().to_string());
                if path.is_valid() {
                    paths.push(path);
                }
            }
        }
        return paths;
    }
}

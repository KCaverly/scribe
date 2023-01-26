use crate::note::Note;
use crate::path::ScribePath;
use crate::NOTES_DIR;
use std::error::Error;
use std::fs::{self, rename};
use skim::prelude::*;
use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;
use walkdir::WalkDir;
use std::io::Write;

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

    pub fn interactive_create() -> Note{
        unimplemented!("Interactive Create not yet implemented!");
    }

    pub fn interactive_transfer() {
        unimplemented!("Interactive Transfer not yet implemented!");
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

    pub fn fuzzy_finder(
        search_options: Vec<String>,
        prompt: &str,
        multi: bool,
    ) -> Vec<Arc<dyn SkimItem>> {
        unimplemented!("Fuzzy Finder Not yet Implemented!");
    }

    pub fn pull() -> bool {
        unimplemented!("Git Pull not yet implemented!");
    }

    pub fn save(commit_message: &str) -> bool {
        unimplemented!("Git Push not yet implemented!");
    }

    pub fn sync(commit_message: Option<String>) -> bool {
        unimplemented!("Git Sync not yet implemented!");
    }
}

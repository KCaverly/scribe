use crate::parsers::embedded_links::EmbeddedLinks;
use crate::parsers::internal_links::InternalLinks;
use crate::parsers::title::Title;
use crate::parsers::web_links::WebLinks;
use crate::parsers::{date::Date, tags::Tags};
use crate::path::ScribePath;
use crate::NOTES_DIR;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct NoteInfo {
    pub path: String,
    pub title: Option<String>,
    pub tags: Option<HashSet<String>>,
    pub date: Option<DateTime<Local>>,
    pub embedded_links: Option<HashSet<String>>,
    pub internal_links: Option<HashSet<String>>,
    pub web_links: Option<HashSet<String>>,
}

impl NoteInfo {
    pub fn parse(path: &ScribePath) -> Self {
        let data = path.get_data();
        let note: Self;
        if data.is_some() {
            let file_data = data.unwrap();
            let title = Title::parse(&file_data);
            let tags = Tags::parse(&file_data);
            let date = Date::parse(&file_data);
            let embedded_links = EmbeddedLinks::parse(&file_data);
            let internal_links = InternalLinks::parse(&file_data);
            let web_links = WebLinks::parse(&file_data);

            note = NoteInfo {
                path: path.as_string(true),
                title,
                tags,
                date,
                embedded_links,
                internal_links,
                web_links,
            }
        } else {
            note = NoteInfo {
                path: path.as_string(true),
                title: None,
                tags: None,
                date: None,
                embedded_links: None,
                internal_links: None,
                web_links: None,
            }
        }

        return note;
    }

    pub fn has_backlink(&self, path: &ScribePath) -> bool {
        if self.internal_links.is_some() {
            let links = self.internal_links.as_ref().unwrap();
            if links.contains(&path.as_string(false)) {
                return true;
            } else if links.contains(&path.as_string(true)) {
                return false;
            }
        }

        if self.embedded_links.is_some() {
            let links = self.embedded_links.as_ref().unwrap();
            if links.contains(&path.as_string(false).replace(".md", "")) {
                return true;
            } else if links.contains(&path.as_string(true).replace(".md", "")) {
                return true;
            }
        }

        return false;
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScribeIndex {
    pub notes: Vec<NoteInfo>,
}

impl ScribeIndex {
    pub fn new() -> Self {
        let notes: Vec<NoteInfo> = vec![];
        return Self { notes };
    }
    pub fn load(index_path: Option<ScribePath>) -> Option<Self> {
        let data: Option<String>;
        if index_path.is_none() {
            data = Self::get_location().get_data();
        } else {
            data = index_path.unwrap().get_data();
        }

        if data.is_some() {
            let index = serde_json::from_str(&data.unwrap());
            if index.is_ok() {
                return Some(index.unwrap());
            }
        }
        return None;
    }
    pub fn index(&mut self) {
        // Iterate Through Notes Folder
        let root = ScribePath::from(&*NOTES_DIR);
        for file in root.get_children() {
            if file.is_markdown() {
                // Parse and Analyze Note
                let note = NoteInfo::parse(&file);
                self.notes.push(note);
            }
        }
    }

    fn get_location() -> ScribePath {
        let mut root = ScribePath::root(None);
        root.extend(".scribe");
        return root;
    }

    pub fn write(&self) {
        let json_str = serde_json::to_string_pretty(&self).unwrap();
        _ = std::fs::write(Self::get_location().as_string(true), json_str);
    }

    pub fn delete(&self, path: &ScribePath) {
        todo!();
    }

    pub fn update(&self, path: &ScribePath) {
        for mut note in &self.notes {
            if note.path == path.as_string(true) {
                let new_note = NoteInfo::parse(&path);
                note = &new_note;
            }
        }
    }

    pub fn get_backlinks(&self, path: &ScribePath) -> Vec<ScribePath> {
        let mut links: Vec<ScribePath> = vec![];
        for note in &self.notes {
            if note.has_backlink(path) {
                links.push(ScribePath::from(&note.path));
            }
        }
        return links;
    }

    pub fn insert(&self, path: &ScribePath) {
        todo!();
    }
}

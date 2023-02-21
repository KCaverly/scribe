use crate::parsers::embedded_links::EmbeddedLinks;
use crate::parsers::internal_links::InternalLinks;
use crate::parsers::title::Title;
use crate::parsers::web_links::WebLinks;
use crate::parsers::{date::Date, tags::Tags};
use crate::path::ScribePath;
use crate::NOTES_DIR;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct NoteInfo {
    path: String,
    title: Option<String>,
    tags: Option<HashSet<String>>,
    date: Option<DateTime<Local>>,
    embedded_links: Option<HashSet<String>>,
    internal_links: Option<HashSet<String>>,
    web_links: Option<HashSet<String>>,
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
                title: title,
                tags: tags,
                date: date,
                embedded_links: embedded_links,
                internal_links: internal_links,
                web_links: web_links,
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
    notes: Vec<NoteInfo>,
}

impl ScribeIndex {
    pub fn new() -> Self {
        let mut notes: Vec<NoteInfo> = vec![];
        return Self { notes: notes };
    }
    pub fn load() -> Option<Self> {
        let data = std::fs::read(Self::get_location().as_string(true));
        if data.is_ok() {
            let unwrapped = data.unwrap();
            let str_data = std::str::from_utf8(&unwrapped);
            let index = serde_json::from_str(str_data.unwrap());
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
        let mut root = ScribePath::root();
        root.extend(".scribe");
        return root;
    }

    pub fn write(&self) {
        let json_str = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(Self::get_location().as_string(true), json_str);
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

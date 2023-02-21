use crate::parsers::embedded_links::EmbeddedLinks;
use crate::parsers::internal_links::InternalLinks;
use crate::parsers::title::Title;
use crate::parsers::web_links::WebLinks;
use crate::parsers::{date::Date, tags::Tags};
use crate::path::ScribePath;
use crate::NOTES_DIR;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize)]
pub struct NoteInfo {
    path: String,
    title: Option<String>,
    tags: Option<HashSet<String>>,
    date: Option<DateTime<Local>>,
    embedded_links: Option<HashSet<String>>,
    internal_links: Option<HashSet<String>>,
    web_links: Option<HashSet<String>>,
}

#[derive(Serialize)]
pub struct ScribeIndex {
    notes: Vec<NoteInfo>,
}

impl ScribeIndex {
    pub fn new() -> Self {
        let mut notes: Vec<NoteInfo> = vec![];
        return Self { notes: notes };
    }
    pub fn index(&mut self) {
        // Iterate Through Notes Folder
        let root = ScribePath::from(&*NOTES_DIR);
        for file in root.get_children() {
            if file.is_markdown() {
                // Parse and Analyze Note
                let data = file.get_data();
                let note: NoteInfo;
                if data.is_some() {
                    let file_data = data.unwrap();
                    let title = Title::parse(&file_data);
                    let tags = Tags::parse(&file_data);
                    let date = Date::parse(&file_data);
                    let embedded_links = EmbeddedLinks::parse(&file_data);
                    let internal_links = InternalLinks::parse(&file_data);
                    let web_links = WebLinks::parse(&file_data);

                    note = NoteInfo {
                        path: file.as_string(true),
                        title: title,
                        tags: tags,
                        date: date,
                        embedded_links: embedded_links,
                        internal_links: internal_links,
                        web_links: web_links,
                    }
                } else {
                    note = NoteInfo {
                        path: file.as_string(true),
                        title: None,
                        tags: None,
                        date: None,
                        embedded_links: None,
                        internal_links: None,
                        web_links: None,
                    }
                }

                self.notes.push(note);
            }
        }
    }

    pub fn write(&self) {
        let json_str = serde_json::to_string_pretty(&self.notes).unwrap();
        std::fs::write("/home/kcaverly/kb/.scribe", json_str);
    }
}

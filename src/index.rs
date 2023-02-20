use crate::parsers::tags::Tags;
use crate::parsers::title::Title;
use crate::path::ScribePath;
use crate::NOTES_DIR;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize)]
pub struct NoteInfo {
    path: String,
    title: Option<String>,
    tags: Option<HashSet<String>>,
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

                    note = NoteInfo {
                        path: file.as_string(true),
                        title: title,
                        tags: tags,
                    }
                } else {
                    note = NoteInfo {
                        path: file.as_string(true),
                        title: None,
                        tags: None,
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

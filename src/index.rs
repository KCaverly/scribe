use crate::config::ScribeConfig;
use crate::parsers::embedded_links::EmbeddedLinks;
use crate::parsers::internal_links::InternalLinks;
use crate::parsers::title::Title;
use crate::parsers::web_links::WebLinks;
use crate::parsers::{date::Date, tags::Tags};
use crate::path::ScribePath;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
        let root = ScribePath::root();
        for file in root.get_children() {
            if file.is_markdown() & !file.is_template() & !file.is_tmp() {
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
        _ = std::fs::write(Self::get_location().as_string(true), json_str);
    }

    pub fn delete(&self, path: &ScribePath) {
        todo!();
    }

    pub fn update(&mut self, path: &ScribePath) {
        for i in 0..self.notes.len() {
            if self.notes[i].path == path.as_string(true) {
                let new_note = NoteInfo::parse(&path);
                self.notes[i] = new_note;
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_note_info_parser() {
        // Create temp file
        let mut new_file = ScribePath::root();
        new_file.extend("tmp/test.md");

        let test_data = r#"---\ntitle: This is a test file\n tags: ["tag1", "tag2"]\n---\n\n# This is a test file\n\nThis is an embedded link [[projects/test/file]]. While this is a web link: [[www.google.com]]"#;

        let res = new_file.create_file(test_data);
        assert!(res.is_ok());

        let test_tags: HashSet<String> = HashSet::from(["tag1".to_string(), "tag2".to_string()]);
        let embedded_links: HashSet<String> = HashSet::from(["projects/test/file".to_string()]);

        let test_note = NoteInfo {
            path: new_file.as_string(true),
            title: Some("This is a test file".to_string()),
            tags: Some(test_tags),
            date: None,
            embedded_links: Some(embedded_links),
            internal_links: None,
            web_links: Some(HashSet::from(["www.google.com".to_string()])),
        };

        let parsed_note = NoteInfo::parse(&new_file);
        assert_eq!(test_note, parsed_note);

        let _res = new_file.delete();
    }

    #[test]
    fn test_note_info_has_backlink() {
        // Create temp file
        let mut new_file = ScribePath::root();
        new_file.extend("tmp/test.md");

        let test_data = r#"---\ntitle: This is a test file\n tags: ["tag1", "tag2"]\n---\n\n# This is a test file\n\nThis is an embedded link [[projects/test/file]]. While this is a web link: [[www.google.com]]"#;

        let res = new_file.create_file(test_data);
        assert!(res.is_ok());

        let parsed_note = NoteInfo::parse(&new_file);

        let mut backlinked_file = ScribePath::root();
        backlinked_file.extend("projects/test/file.md");
        assert!(parsed_note.has_backlink(&backlinked_file));

        let mut not_backlinked_file = ScribePath::root();
        not_backlinked_file.extend("tmp/test.md");
        assert!(!parsed_note.has_backlink(&not_backlinked_file));

        let _res = new_file.delete();
    }

    #[test]
    fn test_index_load_vs_index() {
        let mut index_path = ScribePath::root();
        index_path.extend("test_index.json");
        let loaded_index = ScribeIndex::load(Some(index_path));

        assert!(loaded_index.is_some());

        // Test Creating a New Index
        let mut index = ScribeIndex::new();
        index.index();

        let unwrapped = loaded_index.unwrap();
        for note in &index.notes {
            assert!(unwrapped.notes.contains(&note), "{:?}", note);
        }

        for note in &unwrapped.notes {
            assert!(index.notes.contains(&note), "{:?}", note);
        }

        // Test Updating an Index
        for i in 0..index.notes.len() {
            let note_path = ScribePath::from(&index.notes[i].path);
            if index.notes[i].tags.is_some() {
                let replace_tag = index.notes[i]
                    .tags
                    .as_ref()
                    .unwrap()
                    .iter()
                    .next()
                    .cloned()
                    .unwrap();

                let new_tag = "new_tag";
                let res = note_path.replace(&replace_tag, new_tag);
                assert!(res.is_ok());

                // Update in Index
                index.update(&note_path);

                for j in 0..index.notes.len() {
                    if index.notes[j].path == note_path.as_string(true) {
                        assert!(index.notes[j].tags.as_ref().unwrap().contains("new_tag"));
                    }
                }

                let res2 = note_path.replace("new_tag", &replace_tag);
                assert!(res2.is_ok());
            }
        }

        // let note = &index.notes[0];
        // let note_path = ScribePath::from(&note.path);
        // let replace_tag = note.tags.as_ref().unwrap().iter().next().cloned().unwrap();
        // let new_tag = "new_tag".to_string();
        // note_path.replace(replace_tag.to_owned(), new_tag);
    }
}

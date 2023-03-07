use std::collections::HashMap;

use crate::{
    index::ScribeIndex, path::ScribePath, template::ScribeTemplate, template::ScribeTemplateLibrary,
};

pub struct Note {
    path: ScribePath,
}

impl Note {
    pub fn from_template(
        path: ScribePath,
        template: &ScribeTemplate,
        params: HashMap<String, String>,
    ) -> Option<Self> {
        let init_data = template.fill(&params);
        if init_data.is_ok() {
            let unwrapped = &init_data.unwrap();
            let res = path.create_file(unwrapped);
            if res.is_ok() {
                return Some(Self { path });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn from_path(path: ScribePath) -> Self {
        return Self { path };
    }

    pub fn transfer(&mut self, path: ScribePath) -> Result<(), std::io::Error> {
        // Move File Over
        let og_path = self.path.clone();
        self.path.rename(path)?;

        // Replace Links
        let index = ScribeIndex::load(None);
        if index.is_some() {
            let mut unwrapped_index = index.unwrap();
            let backlinks = unwrapped_index.get_backlinks(&og_path);
            for backlink in backlinks {
                // Update the Backlinked File With Correct Path
                backlink.replace(&og_path.as_string(true), &self.path.as_string(false))?;
                backlink.replace(&og_path.as_string(false), &self.path.as_string(false))?;

                // Update Index
                unwrapped_index.update(&backlink);
            }

            // Delete Existing Note
            unwrapped_index.delete(&og_path);

            // Insert New Note
            unwrapped_index.insert(&self.path);

            // Write New Index
            unwrapped_index.write();
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_from_template() {
        // Create tmp path
        let mut path = ScribePath::root();
        path.extend("tmp/test_new_note.md");

        // Get Basic template
        let library = ScribeTemplateLibrary::load();
        let template = library.get_template("basic").unwrap();

        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("TITLE".to_string(), "this is a test title".to_string());
        params.insert("TAGS".to_string(), r#""tag1", "tag2""#.to_string());
        params.insert("DATE".to_string(), "2022-01-01 12:31 PM".to_string());

        let new_note = Note::from_template(path, template, params);
        assert!(new_note.is_some());
    }

    // #[test]
    // fn test_note_transfer() {
    //     // Get Path of Existing Note
    //     let mut path = ScribePath::root();
    //     path.extend("tmp/test2.md");
    //
    //     let mut new_path = ScribePath::root();
    //     new_path.extend("tmp/test3.md");
    //
    //     let mut note = Note::from_path(path);
    //     let res = note.transfer(new_path);
    //     assert!(res.is_ok());
    // }
}

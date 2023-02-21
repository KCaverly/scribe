use std::collections::HashMap;

use crate::{index::ScribeIndex, path::ScribePath, template::ScribeTemplate};

pub struct Note {
    path: ScribePath,
    data: String,
}

impl Note {
    pub fn from_template(
        path: ScribePath,
        template: &ScribeTemplate,
        params: HashMap<String, String>,
    ) -> Self {
        let init_data = template.fill(&params);
        path.create_file(&init_data);

        return Self {
            path: path,
            data: init_data,
        };
    }

    pub fn from_path(path: ScribePath) -> Self {
        let data = path.get_data();
        if data.is_none() {
            panic!("Please check the file for loading!");
        }
        return Self {
            path: path,
            data: data.unwrap(),
        };
    }

    pub fn transfer(&mut self, category: &str) {
        // Move File Over
        let og_path = self.path.clone();
        let mut new_path = self.path.clone();
        new_path.replace_category(category);
        self.path.rename(new_path);

        // Replace Links
        let index = ScribeIndex::load();
        if index.is_some() {
            let unwrapped_index = index.unwrap();
            let backlinks = unwrapped_index.get_backlinks(&og_path);
            for backlink in backlinks {
                // Update the Backlinked File With Correct Path
                backlink.replace(og_path.as_string(true), self.path.as_string(false));
                backlink.replace(og_path.as_string(false), self.path.as_string(false));

                // Update Index
                unwrapped_index.update(&backlink);
            }

            // Delete Existing Note
            unwrapped_index.delete(&og_path);

            // Insert New Note
            unwrapped_index.insert(&self.path);
        }
    }
}

use std::collections::HashMap;

use crate::{path::ScribePath, template::ScribeTemplate};

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
}

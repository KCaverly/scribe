use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::{fs, str};
use walkdir::WalkDir;

use crate::path::ScribePath;

pub struct ScribeTemplate {
    path: String,
    data: String,
}

impl ScribeTemplate {
    fn get_matches(&self, search_string: &str) -> HashSet<&str> {
        let matcher: Regex = Regex::new(search_string).unwrap();
        matcher
            .find_iter(&self.data)
            .map(|mat| mat.as_str())
            .collect()
    }

    pub fn load(template_path: &str) -> Self {
        let data =
            fs::read(template_path).expect(&format!("Template: {} Does Not Exist!", template_path));

        let template_str = str::from_utf8(&data).expect("Template not valid!");

        return Self {
            path: template_path.to_string(),
            data: template_str.to_string(),
        };
    }

    pub fn from_data(template_path: &str, data: &str) -> Self {
        return Self {
            path: template_path.to_string(),
            data: data.to_string(),
        };
    }

    pub fn get_keys(&self) -> Vec<String> {
        let matches = self.get_matches("\\{\\{\\s([a-zA-Z0-9]+)\\s\\}\\}");

        let mut clean_matches: Vec<String> = Vec::new();
        for key in &matches {
            let clean_match = key.replace("{{ ", "").replace(" }}", "");
            clean_matches.push(clean_match)
        }

        return clean_matches;
    }

    pub fn fill(&self, values: &HashMap<String, String>) -> String {
        let keys = self.get_keys();
        let mut data = self.data.clone();

        for key in keys {
            if !values.contains_key(&key) {
                // TODO: Properly handle this error
                println!("Template Key {} Not included in Values", key);
                panic!("Please Enter All Params");
            } else {
                data = data.replace(&format!(r"{{{{ {} }}}}", key), &values[&key]);
            }
        }

        return data;
    }
}

pub struct ScribeTemplateLibrary {
    templates: HashMap<String, ScribeTemplate>,
}

impl ScribeTemplateLibrary {
    pub fn new() -> Self {
        let mut templates: HashMap<String, ScribeTemplate> = HashMap::new();

        // Basic.md
        let data = include!("templates/basic.md");
        let temp = ScribeTemplate::from_data("templates/basic.md", data);
        templates.insert("basic".to_string(), temp);

        return Self {
            templates: templates,
        };
    }

    pub fn list_templates(&self) -> HashSet<String> {
        let keys: HashSet<String> = self.templates.keys().cloned().collect();
        return keys;
    }

    pub fn get_template(&self, template_name: &str) -> Option<&ScribeTemplate> {
        let template = self.templates.get(template_name);

        if template.is_some() {
            let owned_template = template.unwrap();
            return Some(owned_template);
        }

        return None;
    }
}

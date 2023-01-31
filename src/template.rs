use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::{fs, str};

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

    pub fn get_keys(&self) -> Vec<String> {
        let matches = self.get_matches("\\{\\{\\s([a-zA-Z0-9]+)\\s\\}\\}");

        let mut clean_matches: Vec<String> = Vec::new();
        for key in &matches {
            let clean_match = key.replace("{{ ", "").replace("}}", "");
            clean_matches.push(clean_match)
        }

        return clean_matches;
    }

    pub fn fill(values: HashMap<String, String>) -> String {
        return "".to_string();
    }
}

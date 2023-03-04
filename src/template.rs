use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs, str};

use crate::parsers::parser::Parser;
use crate::path::ScribePath;
use crate::ScribeError;

lazy_static! {
    static ref TEMPLATE_KEYS: Regex = Regex::new(r"\{\{\s([a-zA-Z0-9\_]+)\s\}\}").unwrap();
}

pub struct ScribeTemplate {
    data: String,
}

impl ScribeTemplate {
    pub fn load(template_path: &str) -> Self {
        let data =
            fs::read(template_path).expect(&format!("Template: {} Does Not Exist!", template_path));

        let template_str = str::from_utf8(&data).expect("Template not valid!");

        return Self {
            // path: template_path.to_string(),
            data: template_str.to_string(),
        };
    }

    pub fn from_str(template_str: &str) -> Self {
        return Self {
            data: template_str.to_string(),
        };
    }

    pub fn get_keys(&self) -> Option<HashSet<String>> {
        let matches = Parser::get_matches(&TEMPLATE_KEYS, &self.data);
        return matches;
    }

    pub fn fill(&self, values: &HashMap<String, String>) -> Result<String, ScribeError> {
        let keys = self.get_keys();
        let mut data = self.data.clone();

        for key in keys.unwrap() {
            if !values.contains_key(&key) {
                // TODO: Properly handle this error
                let err = ScribeError::MissingParams;
                return Err(err);
            } else {
                data = data.replace(&format!(r"{{{{ {} }}}}", key), &values[&key]);
            }
        }

        return Ok(data);
    }
}

pub struct ScribeTemplateLibrary {
    templates: HashMap<String, ScribeTemplate>,
}

impl ScribeTemplateLibrary {
    fn builtins() -> HashMap<String, ScribeTemplate> {
        let mut builtin_templates: HashMap<String, ScribeTemplate> = HashMap::new();

        // Basic Template
        builtin_templates.insert(
            "basic".to_string(),
            ScribeTemplate::from_str(
                "---\ntitle: {{ TITLE }}\ndate: {{ DATE }}\ntags: {{ TAGS }}\n---\n\n# {{ TITLE }}",
            ),
        );

        return builtin_templates;
    }

    pub fn load() -> Self {
        let mut templates: HashMap<String, ScribeTemplate> = Self::builtins();

        // Find User Options
        // TODO: Move this Append Path Functionality Up to ScribePath
        let mut template_dir_path: ScribePath = ScribePath::root(None);
        template_dir_path.extend("templates");

        if template_dir_path.exists() {
            for file in template_dir_path.get_children() {
                if file.is_markdown() {
                    let data = file.get_data().unwrap();
                    let template = ScribeTemplate::from_str(&data);
                    templates.insert(file.get_base().unwrap().replace(".md", ""), template);
                }
            }
        }

        return Self { templates };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let _template: ScribeTemplate =
            ScribeTemplate::from_str("# {{ TITLE }}\n\n ## {{ HEADER1 }}");
    }

    #[test]
    fn test_get_keys() {
        let template: ScribeTemplate =
            ScribeTemplate::from_str("# {{ TITLE }}\n\n ## {{ HEADER1 }}");
        let parsed_keys: Option<HashSet<String>> = template.get_keys();

        let mut test_keys: HashSet<String> = HashSet::new();
        test_keys.insert("TITLE".to_string());
        test_keys.insert("HEADER1".to_string());

        assert!(parsed_keys.is_some());
        assert_eq!(parsed_keys.unwrap(), test_keys);
    }

    #[test]
    fn test_get_keys_not_provided() {
        let template: ScribeTemplate = ScribeTemplate::from_str("This is a template without Keys");
        let parsed_keys: Option<HashSet<String>> = template.get_keys();

        assert!(parsed_keys.is_none());
    }

    #[test]
    fn test_template_fill() {
        let template: ScribeTemplate =
            ScribeTemplate::from_str("# {{ TITLE }}\n\nThis is a test template");
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("TITLE".to_string(), "This is the test title".to_string());

        let filled_template = template.fill(&params);
        assert!(filled_template.is_ok());
    }

    #[test]
    fn test_template_missing_params() {
        let template: ScribeTemplate =
            ScribeTemplate::from_str("# {{ TITLE }}\n\nThis is a sample template");
        let params: HashMap<String, String> = HashMap::new();
        let filled_template = template.fill(&params);
        assert!(filled_template.is_err());
    }

    #[test]
    fn test_template_library_load() {
        let _library = ScribeTemplateLibrary::load();
    }

    #[test]
    fn test_template_library_list_templates() {
        let library = ScribeTemplateLibrary::load();
        let templates = library.list_templates();
        assert!(templates.len() > 0);
    }

    #[test]
    fn test_template_library_get_template() {
        let library = ScribeTemplateLibrary::load();
        let templates = library.list_templates();
        for template_name in templates {
            let template = library.get_template(&template_name);
            assert!(template.is_some());
        }
    }
}

use crate::NOTES_DIR;
use std::path::PathBuf;

pub struct ScribePath {
    // Always store the path as the absolute path
    pub path: String,
    pub category: String,
    pub base: Option<String>,
}

impl ScribePath {
    // Constructors
    pub fn new(category: &str, base: &str) -> Self {
        let path = format!(
            "{}/{}/{}",
            &*NOTES_DIR.trim_end_matches("/"),
            category.trim_end_matches("/").trim_start_matches("/"),
            base
        );

        return Self {
            path: path.to_string(),
            category: category.to_string(),
            base: Some(base.to_string()),
        };
    }

    pub fn from(path: &str) -> Self {
        let scribe_path = Self::get_absolute(path);
        let category = Self::get_category(path);
        let base = Self::get_base(path);

        return Self {
            path: scribe_path,
            category: category,
            base: base,
        };
    }

    // Private Methods
    fn get_relative(path: &str) -> String {
        if path.contains(&*NOTES_DIR) {
            return path.replace(&*NOTES_DIR, "");
        }
        return path.to_string();
    }

    fn get_absolute(path: &str) -> String {
        if !path.starts_with(&*NOTES_DIR) {
            return format!("{},{}", &*NOTES_DIR, path.trim_start_matches("/"));
        }
        return path.to_string();
    }

    fn get_base(path: &str) -> Option<String> {
        let rel_path = path.replace(&*NOTES_DIR, "");

        let path_parts = rel_path.split("/").collect::<Vec<&str>>();
        if path_parts.len() == 1 {
            return None;
        } else {
            return Some(path_parts.last().unwrap().to_string());
        }
    }

    fn get_category(path: &str) -> String {
        let relative_path = Self::get_relative(path);
        let base = Self::get_base(path);

        let binding: String;
        if base.is_some() {
            binding = relative_path.replace(&base.unwrap(), "");
        } else {
            binding = relative_path;
        }

        let category = binding.trim_start_matches("/").trim_end_matches("/");
        return category.to_string();
    }

    // Public Methods
    pub fn exists(&self) -> bool {
        return self.as_pathbuf().exists();
    }

    pub fn is_dir(&self) -> bool {
        return self.as_pathbuf().is_dir();
    }

    pub fn is_hidden(&self) -> bool {
        if self.category.contains(".") {
            return true;
        }

        return false;
    }

    pub fn is_valid(&self) -> bool {
        if self.category.len() == 0 {
            return false;
        }
        return true;
    }

    pub fn as_string(&self, absolute: bool) -> String {
        if absolute {
            return Self::get_absolute(&self.path);
        } else {
            return Self::get_relative(&self.path);
        }
    }

    pub fn as_pathbuf(&self) -> PathBuf {
        return PathBuf::from(&self.path);
    }

    pub fn replace_category(&mut self, category: &str) {
        self.category = category.to_string();
    }

    pub fn is_markdown(&self) -> bool {
        if self.base.is_none() {
            return false;
        } else {
            return self.base.as_ref().unwrap().contains(".md");
        }
    }
}

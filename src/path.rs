use crate::NOTES_DIR;
use std::path::PathBuf;

pub struct ScribePath {
    // Always store the path as the absolute path
    pub path: String,
    pub category: String,
    pub base: String,
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
            base: base.to_string(),
        };
    }

    pub fn from(path: &str) -> Self {

        let scribe_path = Self::get_absolute(path);
        let category = Self::get_category(path);
        let base = Self::get_base(path);

        return Self {
            path: scribe_path,
            category: category,
            base: base
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

    fn get_base(path: &str) -> String {
        return path.split("/").last().unwrap().to_string();
    }

    fn get_category(path: &str) -> String {
        let relative_path = Self::get_relative(path);
        let base = Self::get_base(path);
        let binding = relative_path.replace(&base, "");
        let category = binding
            .trim_start_matches("/")
            .trim_end_matches("/");
        return category.to_string();
    }

    // Public Methods
    pub fn exists(&self) -> bool {
        return self.as_pathbuf().exists();
    }

    pub fn is_dir(&self) -> bool {
        return self.as_pathbuf().is_dir();
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
}

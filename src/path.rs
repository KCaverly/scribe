use crate::NOTES_DIR;
use std::{fs, path::PathBuf};

pub struct ScribePath {
    pub path: String,
}

impl ScribePath {
    pub fn new(category: &str, base: &str) -> Self {
        let path = format!(
            "{}/{}/{}",
            &*NOTES_DIR.trim_end_matches("/"),
            category.trim_end_matches("/").trim_start_matches("/"),
            base
        );
        return Self { path: path };
    }

    pub fn from(path: &str) -> Self {
        let scribe_path = Self::get_absolute(path);
        return Self { path: scribe_path };
    }

    fn get_relative(path: &str) -> String {
        if path.contains(&*NOTES_DIR) {
            return path.replace(&*NOTES_DIR, "");
        }
        return path.to_string();
    }

    fn get_absolute(path: &str) -> String {
        if !path.starts_with(&*NOTES_DIR) {
            return format!("{}/{}", &*NOTES_DIR, path.trim_start_matches("/"));
        }
        return path.to_string();
    }

    pub fn get_base(&self) -> Option<String> {
        let relative_path = Self::get_relative(&self.path);
        let path_parts = relative_path.split("/").collect::<Vec<&str>>();
        if path_parts.len() == 1 {
            return None;
        } else {
            return Some(path_parts.last().unwrap().to_string());
        }
    }

    pub fn get_category(&self) -> String {
        let relative_path = Self::get_relative(&self.path);
        let base = self.get_base();

        let binding: String;
        if base.is_some() {
            binding = relative_path.replace(&base.unwrap(), "");
        } else {
            binding = relative_path;
        }

        let category = binding.trim_start_matches("/").trim_end_matches("/");
        return category.to_string();
    }

    pub fn exists(&self) -> bool {
        return self.as_pathbuf().exists();
    }

    pub fn is_dir(&self) -> bool {
        todo!();
    }

    pub fn is_hidden(&self) -> bool {
        todo!();
    }

    pub fn is_valid(&self) -> bool {
        todo!();
    }

    pub fn as_string(&self, absolute: bool) -> String {
        todo!();
    }

    pub fn as_pathbuf(&self) -> PathBuf {
        return PathBuf::from(&self.path);
    }

    pub fn replace_category(&self, category: &str) {
        todo!();
    }

    pub fn is_markdown(&self) -> bool {
        todo!();
    }

    pub fn create_directory(&self) {
        todo!();
    }

    pub fn create_file(&self, data: String) {
        todo!();
    }

    pub fn get_data(&self) -> Option<&[u8]> {
        todo!();
    }
}

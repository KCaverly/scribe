use crate::NOTES_DIR;
use std::io::Write;
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
        return self.as_pathbuf().is_dir();
    }

    pub fn is_hidden(&self) -> bool {
        let file_name = self.get_base();
        if file_name.is_some() {
            if file_name.unwrap().starts_with(".") {
                return true;
            }
        }

        if self.get_category().contains(".") {
            return true;
        }

        return false;
    }

    pub fn is_valid(&self) -> bool {
        todo!();
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
        let existing_category = self.get_category();
        let new_path = self.path.replace(&existing_category, category);
        self.path = new_path;
    }

    pub fn is_markdown(&self) -> bool {
        let file_name = self.get_base();
        if file_name.is_none() {
            return false;
        } else if file_name.unwrap().ends_with(".md") {
            return true;
        }
        return false;
    }

    pub fn get_parent(&self) -> Self {
        let pathbuf = self.as_pathbuf();
        let path_str = pathbuf.parent().unwrap().to_str().unwrap();
        return Self {
            path: path_str.to_string(),
        };
    }

    pub fn create_directory(&self) {
        if !self.exists() {
            let pathbuf = self.as_pathbuf();
            if self.is_dir() {
                _ = fs::create_dir_all(pathbuf);
            } else {
                _ = fs::create_dir_all(pathbuf.parent().unwrap());
            }
        }
    }

    pub fn create_file(&self, data: String) {
        let parent = self.get_parent();
        println!("{}", parent.as_string(true));
        if !parent.exists() {
            self.create_directory();
        }

        let mut file = fs::File::create(self.as_string(true)).expect("Unable to create file!");
        _ = file.write_all(data.trim().as_bytes());
    }

    pub fn delete(&self) {
        if !self.is_dir() {
            fs::remove_file(self.as_string(true));
        } else {
            // Will recursively delete all data in directory.
            fs::remove_dir_all(self.as_string(true));
        }
    }

    pub fn get_data(&self) -> Option<&[u8]> {
        todo!();
    }
}

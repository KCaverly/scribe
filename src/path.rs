use crate::config::ScribeConfig;
use std::fs::rename;
use std::io::{self, Write};
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ScribePath {
    pub path: String,
}

impl ScribePath {
    pub fn new(category: &str, base: &str) -> Self {
        let cfg: ScribeConfig = ScribeConfig::load();
        let dir: String = cfg.get("directory").unwrap().to_string();
        let path = format!(
            "{}/{}/{}",
            &dir.trim_end_matches("/"),
            category.trim_end_matches("/").trim_start_matches("/"),
            base
        );
        return Self { path };
    }

    pub fn from(path: &str) -> Self {
        let scribe_path = Self::get_absolute(path);
        return Self { path: scribe_path };
    }

    pub fn root() -> Self {
        let cfg: ScribeConfig = ScribeConfig::load();
        return Self {
            path: cfg.get("directory").unwrap().to_string(),
        };
    }

    fn get_relative(path: &str) -> String {
        let cfg: ScribeConfig = ScribeConfig::load();
        let dir: String = cfg.get("directory").unwrap().to_string();
        if path.contains(&dir) {
            return path.replace(&dir, "").trim_start_matches("/").to_string();
        }
        return path.to_string();
    }

    fn get_absolute(path: &str) -> String {
        let cfg: ScribeConfig = ScribeConfig::load();
        let dir: String = cfg.get("directory").unwrap().to_string();
        if !path.starts_with(&dir) {
            return format!("{}/{}", &dir, path.trim_start_matches("/"));
        }
        return path.to_string();
    }

    pub fn extend(&mut self, path: &str) {
        self.path = format!("{}/{}", self.path, path).to_string();
    }

    pub fn get_children(&self) -> Vec<ScribePath> {
        let mut paths: Vec<ScribePath> = vec![];
        for entry in WalkDir::new(self.as_string(true))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = ScribePath::from(&entry.path().display().to_string());
            if path.is_valid() {
                paths.push(path);
            }
        }

        return paths;
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

    pub fn is_valid(&self) -> bool {
        if self.as_string(true).contains(".git") {
            return false;
        }

        if self.get_category().len() == 0 {
            return false;
        }

        return true;
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

        if self.get_category().starts_with(".") {
            return true;
        }

        return false;
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

    pub fn is_template(&self) -> bool {
        return self.get_parent().as_string(false).starts_with("templates");
    }

    pub fn is_tmp(&self) -> bool {
        return self.as_string(true).contains("tmp");
    }

    pub fn get_parent(&self) -> Self {
        let pathbuf = self.as_pathbuf();
        let path_str = pathbuf.parent().unwrap().to_str().unwrap();
        return Self {
            path: path_str.to_string(),
        };
    }

    pub fn create_file(&self, data: &str) -> Result<(), io::Error> {
        let file_res: Result<(), io::Error>;
        let parent = self.get_parent();
        if !parent.exists() {
            let dir_res = fs::create_dir_all(parent.as_pathbuf());

            if dir_res.is_ok() {
                let mut file =
                    fs::File::create(self.as_string(true)).expect("Unable to create file!");
                file_res = file.write_all(data.trim().as_bytes());
                return file_res;
            }
        } else {
            let mut file = fs::File::create(self.as_string(true)).expect("Unable to create file!");
            file_res = file.write_all(data.trim().as_bytes());
            return file_res;
        }

        return Ok(());
    }

    pub fn delete(&self) -> Result<(), io::Error> {
        if self.exists() {
            if !self.is_dir() {
                let res = fs::remove_file(self.as_string(true));
                return res;
            } else {
                // Will recursively delete all data in directory.
                let res = fs::remove_dir_all(self.as_string(true));
                return res;
            }
        }

        return Ok(());
    }

    pub fn rename(&mut self, new_path: Self) -> Result<(), io::Error> {
        let res = rename(self.as_string(true), new_path.as_string(true));
        if res.is_ok() {
            self.path = new_path.as_string(true);
        }
        return res;
    }

    pub fn get_data(&self) -> Option<String> {
        let data = fs::read(self.as_string(true));
        if data.is_ok() {
            return Some(
                std::str::from_utf8(&data.unwrap())
                    .ok()
                    .unwrap()
                    .to_string(),
            );
        }
        return None;
    }

    pub fn replace(&self, replace_str: String, new_str: String) -> std::io::Result<()> {
        let og_data = self.get_data().unwrap();
        let new_data = og_data.replace(&replace_str, &new_str);
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.as_string(true))?;
        f.write_all(new_data.as_bytes())?;
        f.flush()?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_path_new() {
        let _path = ScribePath::new("test", "test_base");
    }

    #[test]
    fn test_path_from() {
        let _path = ScribePath::from("/tmp/");
    }

    #[test]
    fn test_path_root() {
        let _path = ScribePath::root();
    }

    #[test]
    fn test_path_extend() {
        let test_path = "/home/tmp";
        let mut path = ScribePath::from(test_path);
        path.extend("test");

        let test_path = ScribePath::from("/home/tmp/test");
        assert_eq!(path, test_path);
    }

    #[test]
    fn test_path_get_children() {
        let root = ScribePath::root();
        let children = root.get_children();

        let mut test_child = ScribePath::root();
        assert!(!children.contains(&test_child));

        test_child.extend("inbox/test_file1.md");
        assert!(children.contains(&test_child));
    }

    #[test]
    fn test_path_get_base() {
        let root = ScribePath::new("test_category", "test_base");
        assert_eq!(root.get_base().unwrap(), "test_base");
        assert_ne!(root.get_base().unwrap(), "teasasdf");
    }

    #[test]
    fn test_path_get_category() {
        let root = ScribePath::new("test_category", "test_base");
        assert_eq!(root.get_category(), "test_category");
        assert_ne!(root.get_category(), "asdfasdf");
    }

    #[test]
    fn test_path_exists() {
        let mut root = ScribePath::root();
        assert!(root.exists());

        root.extend("asasdfasdf");
        assert!(!root.exists());
    }

    #[test]
    fn test_path_is_valid() {
        let path = ScribePath::new("test_category", "test.md");
        assert!(path.is_valid());

        let root = ScribePath::root();
        assert!(!root.is_valid());

        let path2 = ScribePath::new("test_category", "index.git");
        assert!(!path2.is_valid());
    }

    #[test]
    fn test_path_is_dir() {
        let mut root = ScribePath::root();
        assert!(root.is_dir());

        root.extend("test_file1.md");
        assert!(!root.is_dir());
    }

    #[test]
    fn test_path_is_hidden() {
        let mut root = ScribePath::root();
        root.extend("test_file.md");

        assert!(!root.is_hidden());

        let hidden_dir = ScribePath::new(".dotfiles", "tmux.conf");
        assert!(hidden_dir.is_hidden());

        let hidden_file = ScribePath::new("category", ".gitignore");
        assert!(hidden_file.is_hidden());
    }

    #[test]
    fn test_path_as_string() {
        let mut root = ScribePath::root();
        let cfg = ScribeConfig::load();
        let dir = cfg.get("directory").unwrap();

        assert_eq!(root.as_string(true), *dir);

        root.extend("test");
        assert_eq!(root.as_string(true), format!("{}/{}", dir, "test"));

        assert_eq!(root.as_string(false), "test");
    }

    #[test]
    fn test_path_as_pathbuf() {
        let cfg = ScribeConfig::load();
        let dir = cfg.get("directory").unwrap();
        let pathbuf = PathBuf::from(dir);
        let path = ScribePath::from(dir);
        assert_eq!(pathbuf, path.as_pathbuf());
    }

    #[test]
    fn test_path_replace_category() {
        let mut path = ScribePath::new("first", "test");
        assert_eq!(path.get_category(), "first");

        path.replace_category("second");
        assert_eq!(path.get_category(), "second");
        assert_ne!(path.get_category(), "first");
    }

    #[test]
    fn test_path_is_markdown() {
        let md_path = ScribePath::new("category", "test.md");
        assert!(md_path.is_markdown());
        let txt_path = ScribePath::new("category", "test.txt");
        assert!(!txt_path.is_markdown());
    }

    #[test]
    fn test_path_get_parent() {
        let root = ScribePath::root();
        let mut path = ScribePath::root();
        path.extend("test");

        assert_eq!(root, path.get_parent());
    }

    #[test]
    fn test_path_create_and_delete_file() {
        let mut root = ScribePath::root();
        root.extend("tmp/test_asdfasdf.md");

        let res = root.create_file("this is test data");
        assert!(res.is_ok());

        let res = root.delete();
        assert!(res.is_ok());
    }

    #[test]
    fn test_path_create_rename_and_delete_file() {
        let mut root = ScribePath::root();
        root.extend("tmp/test2.md");

        let mut new_file = ScribePath::root();
        new_file.extend("tmp/test2_renamed.md");

        let res = root.create_file("this is test data");
        assert!(res.is_ok());

        let res = root.rename(new_file);
        assert!(res.is_ok());

        let res = root.delete();
        assert!(res.is_ok());
    }

    #[test]
    fn test_path_get_data() {
        let mut root = ScribePath::root();
        root.extend("tmp/test3.md");

        let test_data = "This is a test file".to_string();
        let res = root.create_file(&test_data);
        assert!(res.is_ok());

        let get_data = root.get_data();
        assert!(get_data.is_some());
        assert_eq!(get_data.unwrap(), test_data);

        let delete_res = root.delete();
        assert!(delete_res.is_ok());
    }

    #[test]
    fn test_path_replace() {
        let mut root = ScribePath::root();
        root.extend("tmp/test4.md");

        let test_data = "This is a test file".to_string();
        let res = root.create_file(&test_data);
        assert!(res.is_ok());

        let res = root.replace("test".to_string(), "tested!".to_string());
        assert!(res.is_ok());
        let data = root.get_data();
        assert!(data.is_some());
        assert_eq!(data.unwrap(), "This is a tested! file".to_string());

        let delete_res = root.delete();
        assert!(delete_res.is_ok());
    }
}

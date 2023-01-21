use crate::NOTES_DIR;
use std::path::PathBuf;

pub struct PJPath {
    pub path: String,
    pub category: String,
    pub base: String,
}

impl PJPath {
    pub fn exists(&self) -> bool {
        let pathbuf = PathBuf::from(&self.path);
        return pathbuf.exists();
    }

    pub fn parse(path: &str) -> Self {
        let pathbuf = PathBuf::from(path);

        let relative_path = path.replace(&*NOTES_DIR, "");
        let base = relative_path.split("/").last().unwrap();

        return Self {
            path: path.to_string(),
            category: relative_path
                .replace(base, "")
                .trim_end_matches("/")
                .trim_start_matches("/")
                .to_string(),
            base: base.to_string(),
        };
    }

    pub fn replace_category(&mut self, category: &str) {
        self.path = self.path.replace(&self.category, category);
        self.category = category.to_string();
    }

    pub fn relative_path(&self) -> String {
        return self.path.replace(&*NOTES_DIR, "").to_string();
    }
}

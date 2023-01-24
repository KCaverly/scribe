use crate::note::Note;
use crate::path::ScribePath;
use std::fs::rename;

pub struct Scribe {}

impl Scribe {

    // Public Functions
    pub fn create(title: String, category: Option<String>, tags: Option<Vec<String>>) {
        let note = Note::new(title, category, tags);
        note.init();
    }

    pub fn transfer(path: &str, category: &str) -> std::io::Result<()> {

        // Transfer File Over
        let old_path = ScribePath::from(path);
        let mut new_path = ScribePath::from(path);
        new_path.replace_category(category);

        if !old_path.exists() {
            // TODO: Update error handling if path you are trying to move doesnt exist
            return Ok(());
        }

        rename(&old_path.as_string(), &new_path.as_string());

        // Transfer Links
        let results = Self::search(format!("[A-Za-z0-9/]+?{}", old_path.as_string(false)));
        if results.is_ok() {
            for res in results.unwrap() {
                // TODO: Move this into a note::Note public function
                let og_data = fs::read_to_string(&res.0).unwrap();
                let new_data = og_data.replace(&res.1, &new_path.relative_path());

                let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open(&res.0)?;
                f.write_all(new_data.as_bytes())?;
                f.flush()?;
            }
        }
        Ok(())

    }

    pub fn interactive_create() {
        unimplemented!("Interactive Create not yet implemented!");
    }

    pub fn interactive_transfer() {
        unimplemented!("Interactive Transfer not yet implemented!");
    }

    pub fn search(search_string: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {}

    pub fn fuzzy_finder(search_options: Vec<String>, prompt: &str, multi: bool) -> Vec<Arc<dyn SkimItem>> {}

    pub fn pull() -> bool {
        unimplemented!("Git Pull not yet implemented!");
    }

    pub fn save(commit_message: &str) -> bool {
        unimplemented!("Git Push not yet implemented!");
    }
    

}

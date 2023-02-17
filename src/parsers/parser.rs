use std::collections::HashSet;

use regex::Regex;

pub struct Parser {
    search_string: String,
}

impl Parser {
    pub fn new(search_string: String) -> Self {
        Self {
            search_string: search_string,
        }
    }

    pub fn get_matches<'a>(&'a self, data: &'a str) -> Option<HashSet<&str>> {
        let re: Regex = Regex::new(&self.search_string).unwrap();
        let caps = re.captures_iter(data);
        let res: HashSet<&str> = caps
            .map(|m| m.get(1))
            .map(|x| x.expect("Failed").as_str())
            .collect();

        return Some(res);
    }
}

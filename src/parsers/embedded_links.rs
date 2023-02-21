use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct EmbeddedLinks {}

impl EmbeddedLinks {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        let search = "\\[\\[([a-zA-Z0-9/\\s_\\-]+)[|]?[\\]\\]]?".to_string();
        let parser = Parser::new(search);
        let matches = parser.get_matches(data);

        if matches.is_some() {
            let mut full_matches: HashSet<String> = HashSet::new();
            for match_ in matches.unwrap() {
                full_matches.insert(match_.trim().to_string());
            }
            return Some(full_matches);
        }
        return None;
    }
}

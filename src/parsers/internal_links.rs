use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct InternalLinks {}

impl InternalLinks {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        let search = "\\[.+\\]\\(([A-Za-z0-9\\-/]+)\\)".to_string();
        let parser = Parser::new(search);
        let matches = parser.get_matches(data);

        let mut full_matches: HashSet<String> = HashSet::new();
        if matches.is_some() {
            for match_ in matches.unwrap() {
                if !match_.starts_with("http") & !match_.starts_with("www.") {
                    full_matches.insert(match_);
                }
            }
        }
        return Some(full_matches);
    }
}

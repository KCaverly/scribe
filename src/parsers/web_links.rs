use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct WebLinks {}

impl WebLinks {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        // Search for http... links
        let search = "(http[s]?:[^\\s\\)\\]]+)".to_string();
        let parser = Parser::new(search);
        let matches = parser.get_matches(data);

        // Search for www.links
        let search = "(www.[^\\s\\)\\]]+)".to_string();
        let parser = Parser::new(search);
        let matches_2 = parser.get_matches(data);

        let mut full_matches: HashSet<String> = HashSet::new();
        if matches.is_some() {
            for match_ in matches.unwrap() {
                full_matches.insert(match_);
            }
        }

        if matches_2.is_some() {
            for match_ in matches_2.unwrap() {
                if !full_matches.contains(&format!("https://{}", match_))
                    & !full_matches.contains(&format!("http://{}", match_))
                {
                    full_matches.insert(match_);
                }
            }
        }

        return Some(full_matches);
    }
}

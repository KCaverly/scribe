use fancy_regex::Regex;
use lazy_static::lazy_static;

use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct WebLinks {}

impl WebLinks {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        lazy_static! {
            static ref WEB_LINKS_1: Regex = Regex::new("(http[s]?:[^\\s\\)\\]]+)").unwrap();
            static ref WEB_LINKS_2: Regex = Regex::new("(www.[^\\s\\)\\]]+)").unwrap();
        };

        // Search for http... links
        let matches = Parser::get_matches(&WEB_LINKS_1, data);

        // Search for www.links
        let matches_2 = Parser::get_matches(&WEB_LINKS_2, data);

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

        if full_matches.len() == 0 {
            return None;
        }

        return Some(full_matches);
    }
}

use fancy_regex::Regex;
use lazy_static::lazy_static;

use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct EmbeddedLinks {}

impl EmbeddedLinks {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        lazy_static! {
            static ref EMBEDDED_LINKS: Regex =
                Regex::new("\\[\\[([a-zA-Z0-9/\\s_\\-]+)[|]?[\\]\\]]?").unwrap();
        };
        let matches = Parser::get_matches(&EMBEDDED_LINKS, data);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut test_links: HashSet<String> = HashSet::new();
        test_links.insert("projects/test/scribe".to_string());
        test_links.insert("projects/test/scribe/sample".to_string());

        // Test 1
        let test_string = "This is a random note\n This is a [[projects/test/scribe]] \n This is another one [[projects/test/scribe/sample | title]] ";
        let matches = EmbeddedLinks::parse(test_string);
        assert!(matches.is_some());
        let unwrapped = matches.unwrap();
        println!("{:?}", unwrapped);
        assert!(unwrapped.len() == 2);
        assert_eq!(unwrapped, test_links);
    }
}

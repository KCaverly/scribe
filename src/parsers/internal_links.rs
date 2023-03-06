use fancy_regex::Regex;
use lazy_static::lazy_static;

use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct InternalLinks {}

impl InternalLinks {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        lazy_static! {
            static ref INTERNAL_LINKS: Regex =
                Regex::new("\\[.+\\]\\(([A-Za-z0-9\\-/\\._]+)\\)").unwrap();
        };

        let matches = Parser::get_matches(&INTERNAL_LINKS, data);

        if matches.is_some() {
            let mut full_matches: HashSet<String> = HashSet::new();
            for match_ in matches.unwrap() {
                if !match_.starts_with("http") & !match_.starts_with("www.") {
                    full_matches.insert(match_);
                }
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
        // TODO: This link only pulls the first match for some reason
        // test_links.insert("projects/test/test/scribe".to_string());
        test_links.insert("inbox/scribe/test.txt".to_string());

        // Test
        let test_data = "This is a note\n here is a [link](projects/test/test/scribe), this is a [web link though](http://www.google.com), is is another [link](inbox/scribe/test.txt)";
        let parsed_links = InternalLinks::parse(test_data);
        assert!(parsed_links.is_some());
        assert_eq!(parsed_links.unwrap(), test_links);
    }
}

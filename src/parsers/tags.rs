use fancy_regex::Regex;
use lazy_static::lazy_static;

use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct Tags {}

impl Tags {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        lazy_static! {
            static ref TAGS_1: Regex = Regex::new("\\btags:\\s\\[(.+)\\]").unwrap();
            static ref TAGS_2: Regex = Regex::new(r"(?<!')\#([A-Z0-9a-z\-\_]+)").unwrap();
        };

        // Get Front Matter Tags
        let matches = Parser::get_matches(&TAGS_1, data);

        let mut tags = HashSet::<String>::new();
        if matches.is_some() {
            for match_ in matches.unwrap() {
                if match_.contains(",") {
                    for tag_item in match_.split(",") {
                        tags.insert(
                            tag_item
                                .trim()
                                .trim_start_matches(r#"""#)
                                .trim_end_matches(r#"""#)
                                .to_string(),
                        );
                    }
                } else {
                    tags.insert(
                        match_
                            .trim()
                            .trim_start_matches(r#"""#)
                            .trim_end_matches(r#"""#)
                            .to_string(),
                    );
                }
            }
        }

        // Get Hashtag Tags
        let matches = Parser::get_matches(&TAGS_2, data);

        if matches.is_some() {
            for match_ in matches.unwrap() {
                tags.insert(match_.to_string());
            }
        }
        return Some(tags);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_string = r#"tags: ["test", "test2"]\n # this is the title\n #tag2 '#ifdebug'"#;
        let mut test_tags: HashSet<String> = HashSet::new();
        test_tags.insert("test".to_string());
        test_tags.insert("test2".to_string());
        test_tags.insert("tag2".to_string());

        let parsed_tags = Tags::parse(test_string);
        assert!(parsed_tags.is_some());
        assert_eq!(parsed_tags.unwrap(), test_tags);

        let test_string2 = r"#tag3";
        let mut test_tags: HashSet<String> = HashSet::new();
        test_tags.insert("tag3".to_string());

        let parsed_tags = Tags::parse(test_string2);
        assert!(parsed_tags.is_some(), "{:?}", parsed_tags);
        assert_eq!(parsed_tags.unwrap(), test_tags);
    }
}

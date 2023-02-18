use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct Tags {}

impl Tags {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        // Get Front Matter Tags
        let search = "tags:\\s\\[(.+)\\]".to_string();
        let parser = Parser::new(search);
        let matches = parser.get_matches(data);

        let mut tags = HashSet::<String>::new();
        for match_ in matches.unwrap() {
            if match_.contains(",") {
                for tag_item in match_.split(",") {
                    tags.insert(
                        tag_item
                            .trim_start_matches(r#"""#)
                            .trim_end_matches(r#"""#)
                            .to_string(),
                    );
                }
            }
        }

        // Get Hashtag Tags
        let search = r"#([A-Z0-9a-z]+)".to_string();
        let parser = Parser::new(search);
        let matches = parser.get_matches(data);

        for match_ in matches.unwrap() {
            tags.insert(match_.to_string());
        }
        return Some(tags);
    }
}

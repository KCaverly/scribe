use crate::parsers::parser::Parser;
use std::collections::HashSet;

pub struct EmbeddedLinks {}

impl EmbeddedLinks {
    pub fn parse(data: &str) -> Option<HashSet<String>> {
        let search = "\\[\\[([a-zA-Z0-9/\\s]+)[|]?.+\\]\\]".to_string();
        let parser = Parser::new(search);
        let matches = parser.get_matches(data);
        return matches;
    }
}

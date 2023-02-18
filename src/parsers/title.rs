use crate::parsers::parser::Parser;

pub struct Title {}

impl Title {
    pub fn parse(data: &str) -> Option<String> {
        // Default to Front Matter Title
        let parser = Parser::new("title: (.+)".to_string());
        let matches = parser.get_matches(data);

        let title: String;
        if matches.is_some() {
            title = matches.unwrap().into_iter().next().unwrap().to_string();
            return Some(title);
        }

        // Get Title from # header
        let parser = Parser::new("# (.+)".to_string());
        let matches = parser.get_matches(data);
        if matches.is_some() {
            title = matches.unwrap().into_iter().next().unwrap().to_string();
            return Some(title);
        }
        return None;
    }
}

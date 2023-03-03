use crate::parsers::parser::Parser;

pub struct Title {}

impl Title {
    pub fn parse(data: &str) -> Option<String> {
        // Default to Front Matter Title
        let parser = Parser::new("title: (.+)".to_string());
        let matches = parser.get_matches(data);

        let title: String;
        if matches.is_some() {
            let found_matches = matches.unwrap();
            if found_matches.len() > 0 {
                title = found_matches.into_iter().next().unwrap().trim().to_string();
                return Some(title);
            }
        }

        // Get Title from # header
        let parser = Parser::new("(?<!\\#)\\#{1} (.+)".to_string());
        let matches = parser.get_matches(data);
        if matches.is_some() {
            let found_matches = matches.unwrap();
            if found_matches.len() > 0 {
                title = found_matches.into_iter().next().unwrap().trim().to_string();
                return Some(title);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::Title;

    #[test]
    fn test_parse() {
        let test_string = "---\ntitle: This is the title";
        let parsed_title = Title::parse(test_string);
        assert!(parsed_title.is_some());
        assert_eq!(parsed_title.unwrap(), "This is the title".to_string());

        let test_string2 = "# This is the title\n## This is the first header";
        let parsed_title2 = Title::parse(test_string2);
        assert!(parsed_title2.is_some());
        assert_eq!(parsed_title2.unwrap(), "This is the title".to_string());
    }
}

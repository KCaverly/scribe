use std::collections::HashSet;

use regex::Regex;

pub struct Parser {
    search_string: String,
}

impl Parser {
    pub fn new(search_string: String) -> Self {
        Self {
            search_string: search_string,
        }
    }

    pub fn get_matches<'a>(&'a self, data: &'a str) -> Option<HashSet<String>> {
        let re: Regex = Regex::new(&self.search_string).unwrap();
        let caps = re.captures_iter(data);
        let res: HashSet<String> = caps
            .map(|m| m.get(1))
            .map(|x| x.expect("Failed").as_str().trim().to_string())
            .collect();

        return Some(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_matches() {
        let test_string = "Hi my name is Finn!";
        let parser = Parser::new("Hi my name is (.+)".to_string());
        let matches = parser.get_matches(test_string);

        assert!(matches.is_some());
        assert!(matches.unwrap().contains("Finn!"));
    }
}

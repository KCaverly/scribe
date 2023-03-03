use std::collections::HashSet;

use fancy_regex::Regex;

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
        let mut res: HashSet<String> = HashSet::new();

        for cap in re.captures_iter(data) {
            if cap.is_ok() {
                res.insert(cap.unwrap().get(1).unwrap().as_str().trim().to_string());
            }
        }

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

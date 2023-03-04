use std::collections::HashSet;

use fancy_regex::Regex;

pub struct Parser {}

impl Parser {
    pub fn get_matches<'a>(matcher: &'a Regex, data: &'a str) -> Option<HashSet<String>> {
        let mut res: HashSet<String> = HashSet::new();

        for cap in matcher.captures_iter(data) {
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
        let re: Regex = Regex::new("Hi my name is (.+)").unwrap();
        let matches = Parser::get_matches(&re, test_string);

        assert!(matches.is_some());
        assert!(matches.unwrap().contains("Finn!"));
    }
}

use crate::parsers::parser::Parser;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub struct Date {}

impl Date {
    pub fn parse(data: &str) -> Option<DateTime<Local>> {
        let parser = Parser::new("\\bdate: (.+)".to_string());
        let matches = parser.get_matches(data);

        let date_str: String;
        if matches.is_some() {
            let found_matches = matches.unwrap();
            if found_matches.len() > 0 {
                date_str = found_matches
                    .into_iter()
                    .next()
                    .unwrap()
                    .trim_start_matches(r#"""#)
                    .trim_end_matches(r#"""#)
                    .to_string();
                let naive = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %I:%M %p");
                if naive.is_ok() {
                    return Some(Local.from_local_datetime(&naive.unwrap()).unwrap());
                }
                return None;
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let naive_date =
            NaiveDateTime::parse_from_str("2020-01-01 11:59 PM", "%Y-%m-%d %I:%M %p").unwrap();
        let test_date = Local.from_local_datetime(&naive_date).unwrap();

        // Test 1
        let test_data = "date: 2020-01-01 11:59 PM";
        let parsed_date = Date::parse(test_data);
        assert_eq!(test_date, parsed_date.unwrap());

        // Test 2
        let test_data = "date: 2023-12-31 10:52 PM";
        let parsed_date = Date::parse(test_data);
        assert_ne!(test_date, parsed_date.unwrap());

        // Test 3
        let test_data = "Datesas: 2023-12-31 11:22 PM sdf";
        let parsed_date = Date::parse(test_data);
        assert!(parsed_date.is_none());
    }
}

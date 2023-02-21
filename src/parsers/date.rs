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

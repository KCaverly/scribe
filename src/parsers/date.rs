use crate::parsers::parser::Parser;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub struct Date {}

impl Date {
    pub fn parse(data: &str) -> Option<DateTime<Local>> {
        let parser = Parser::new("date: (.+)".to_string());
        let matches = parser.get_matches(data);

        let date_str: String;
        if matches.is_some() {
            date_str = matches.unwrap().into_iter().next().unwrap().to_string();
            let naive = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %I:%M %p").unwrap();
            return Some(Local.from_local_datetime(&naive).unwrap());
        }

        return None;
    }
}

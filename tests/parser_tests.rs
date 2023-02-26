use scribe::{
    index::ScribeIndex,
    parsers::{
        date::Date, embedded_links::EmbeddedLinks, internal_links::InternalLinks, tags::Tags,
        title::Title, web_links::WebLinks,
    },
    path::ScribePath,
};

fn get_test_index() -> ScribeIndex {
    let root =
        ScribePath::from("/home/kcaverly/personal/scribe/examples/small_project/test_index.json");
    let index = ScribeIndex::load(Some(root));
    return index.unwrap();
}

#[test]
fn test_parser_title() {
    let index = get_test_index();

    for note in index.notes {
        let data = ScribePath::from(&note.path).get_data().unwrap();
        let parsed_title = Title::parse(&data);
        assert_eq!(parsed_title, note.title);
    }
}

#[test]
fn test_parser_tags() {
    let index = get_test_index();

    for note in index.notes {
        let data = ScribePath::from(&note.path).get_data().unwrap();
        let parsed_tags = Tags::parse(&data);
        assert_eq!(parsed_tags, note.tags);
    }
}

#[test]
fn test_parser_date() {
    let index = get_test_index();
    for note in index.notes {
        let data = ScribePath::from(&note.path).get_data().unwrap();
        let parsed_date = Date::parse(&data);
        assert_eq!(parsed_date, note.date);
    }
}

#[test]
fn test_parser_embedded_links() {
    let index = get_test_index();
    for note in index.notes {
        let data = ScribePath::from(&note.path).get_data().unwrap();
        let parsed_embedded_links = EmbeddedLinks::parse(&data);
        assert_eq!(parsed_embedded_links, note.embedded_links);
    }
}

#[test]
fn test_parser_internal_links() {
    let index = get_test_index();
    for note in index.notes {
        let data = ScribePath::from(&note.path).get_data().unwrap();
        let parsed_internal_links = InternalLinks::parse(&data);
        assert_eq!(parsed_internal_links, note.internal_links);
    }
}

#[test]
fn test_parser_web_links() {
    let index = get_test_index();
    for note in index.notes {
        let data = ScribePath::from(&note.path).get_data().unwrap();
        let parsed_web_links = WebLinks::parse(&data);
        assert_eq!(parsed_web_links, note.web_links);
    }
}

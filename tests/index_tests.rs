use scribe::index::ScribeIndex;

#[test]
fn test_index_create() {
    let mut index = ScribeIndex::new();
    index.index();
    index.write();
}

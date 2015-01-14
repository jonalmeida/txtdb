// Integration tests

extern crate txtdb;

use txtdb::controller::*;
use txtdb::factory::*;

#[test]
fn test_create_record() {
    let input = String::from_str("1 payload metadata");
    let expected = Record {
        id: 1,
        payload: String::from_str("payload"),
        metadata: String::from_str("metadata"),
    };
    let reader = Reader::new(&Path::new("tests/base-test.txt"));
    let factory: Factory = RecordFactory::new(reader);
    let output: Record = factory.create(input).ok().expect("Parsing failed.");
    assert_eq!(expected.id, output.id);
    assert_eq!(expected.payload, output.payload);
    assert_eq!(expected.metadata, output.metadata);
}

#[test]
fn test_create_from_encoded() {
    let input = String::from_str("5 cGF5bG9hZA== bWV0YWRhdGE=");
    let expected = Record {
        id: 5,
        payload: String::from_str("payload"),
        metadata: String::from_str("metadata"),
    };
    let reader = Reader::new(&Path::new("tests/base-test.txt"));
    let factory: Factory = RecordFactory::new(reader);
    let output: Record = factory.create_from_enc(input).ok().expect("Parsing failed.");
    assert_eq!(expected.id, output.id);
    assert_eq!(expected.payload, output.payload);
    assert_eq!(expected.metadata, output.metadata);
}


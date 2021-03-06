#![allow(dead_code, unused_must_use, unused_imports, unstable)]
use std::str::FromStr;
use controller::Reader;
use utils;

// Each line in the database file is a 'Record' that contains the following data to be read
// and serialized.
pub struct Record {
    // Unique (?) id given to each record.
    pub id: u64,
    // The payload of each record that is Base64 encoded and JSON serialized.
    pub payload: String,
    // Any necessary metedata needed to identify the record.
    // TODO Base64 encoded and JSON encoded?
    pub metadata: String,
}

pub struct Factory {
    reader: Reader,
}

pub type RecordResult<T, E> = Result<T, E>;

//TODO Add some factory traits here if necessary
pub trait RecordFactory {
    fn new(reader: Reader) -> Self;

    fn create(&self, data: String) -> RecordResult<Record, String>;

    fn create_from_enc(&self, data: String) -> RecordResult<Record, String>;
}

impl RecordFactory for Factory {

    fn new(reader: Reader) -> Factory {
        Factory {
            reader: reader,
        }
    }

    fn create(&self, data: String) -> RecordResult<Record, String> {
        let vec_of_data = utils::string_slice(data);

        let id_num = FromStr::from_str(&*vec_of_data[0]);
        let id_value: u64 = match id_num {
            Ok(value)       => value,
            Err(..)         => -1 // This is a failure value
        };

        Ok(Record {
            id: id_value,
            payload: vec_of_data[1].clone(),
            metadata: vec_of_data[2].clone(),
        })
    }

    fn create_from_enc(&self, data: String) -> RecordResult<Record, String> {
        let vec_of_data = utils::string_slice(data);

        let id_num = FromStr::from_str(&*vec_of_data[0]);
        let id_value: u64 = match id_num {
            Ok(value)     => value,
            Err(..)       => -1, // This is a failure value
        };

        let enc_payload = String::from_str(&*vec_of_data[1]);
        let payload = utils::decode_record(enc_payload);
        let enc_metadata = String::from_str(&*vec_of_data[2]);
        let metadata = utils::decode_record(enc_metadata);

        Ok(Record {
            id: id_value,
            payload: payload,
            metadata: metadata,
        })
    }
}

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

use std::str::FromStr;
use controller::Reader;
use utils;

// Each line in the database file is a 'Record' that contains the following data to be read
// and serialized.
pub struct Record {
    // Unique (?) id given to each record.
    id: u64,
    // The payload of each record that is Base64 encoded and JSON serialized.
    payload: String,
    // Any necessary metedata needed to identify the record.
    // TODO Base64 encoded and JSON encoded?
    metadata: String,
}

pub struct Factory {
    reader: Reader,
}

pub type RecordResult<T, E> = Result<T, E>;

//TODO Add some factory traits here if necessary
pub trait RecordFactory {
    fn create(data: String) -> RecordResult<Record, String>;
}

impl Factory {
    pub fn new(reader: Reader) -> Factory {
        Factory {
            reader: reader,
        }
    }
}

//TODO Remove the below flag when `create` is used
#[allow(dead_code, unused_must_use, unused_variables)]
fn create(data: String) -> RecordResult<Record, String> {
    let vec_of_data = utils::string_slice(data);
    let id_num = FromStr::from_str(vec_of_data[0].as_slice());
    let id_value: u64 = match id_num {
        Some(value)     => value,
        None            => -1 // This is a failure value
    };

    Ok(Record {
        id: id_value,
        payload: vec_of_data[1].clone(),
        metadata: vec_of_data[2].clone(),
    })

}

#[test]
fn test_create_record() {
    let input = String::from_str("1 payload metadata");
    let id_: u64 = 1;
    let expected = Record {
        id: id_,
        payload: String::from_str("payload"),
        metadata: String::from_str("metadata"),
    };
    let output: Record = create(input).ok().expect("Parsing failed.");
    assert_eq!(expected.id, output.id);
    assert_eq!(expected.payload, output.payload);
    assert_eq!(expected.metadata, output.metadata);
}

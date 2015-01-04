
// Each line in the database file is a 'Record' that contains the following data to be read
// and serialized.
struct Record {
    // Unique (?) id given to each record.
    id: u64,
    // The payload of each record that is Base64 encoded and JSON serialized.
    payload: String,
    // Any necessary metedata needed to identify the record. TODO Base64 encoded and JSON encoded?
    metadata: String,
}

pub type RecordResult<T, E> = Result<T, E>;

pub trait RecordFactory {
    //TODO Add some factory traits here if necessary
    fn create(data: String) -> RecordResult<String, String>;
}

fn create(data: String) -> RecordResult<String, String> {
    Ok("all guuuud".to_string())
}

#[test]
fn test_create_record() {
    create("foo".to_string());
}

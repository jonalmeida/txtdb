use factory::{Record, RecordResult};

mod controller;
mod factory;
mod utils;

// Add a new record to the database. Returns the id of the record added.
// TODO change the type of the `record` to something else generic
#[allow(dead_code, unused_must_use, unused_variables)]
fn add(record: String) -> RecordResult<u64, String> {
    Ok(1)
}

// Removes a record with the id provided if it exists.
// Returns a `RecordResult` of the record removed.
#[allow(dead_code, unused_must_use, unused_variables)]
fn remove_id(id: u64) -> RecordResult<Record, String> {
    Err("Not implemented yet.".to_string())
}

// Finds and removes the first instance of a record that matches the one provided.
// Returns the id of the record it removes.
#[allow(dead_code, unused_must_use, unused_variables)]
fn remove(record: Record) -> RecordResult<u64, String> {
    Err("Not implemented yet.".to_string())
}

// Searches for a record with the id provided.
// Returns a copy of the record.
#[allow(dead_code, unused_must_use, unused_variables)]
fn find_id(id: u64) -> RecordResult<Record, String> {
    Err("Not implemented yet.".to_string())
}

// Searches for the first instance of a record that matches the one provided.
// Returns the id of the record in the database.
#[allow(dead_code, unused_must_use, unused_variables)]
fn find(record: Record) -> RecordResult<u64, String> {
    Err("Not implemented yet.".to_string())
}

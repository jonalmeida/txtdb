//#![allow(dead_code, unused_must_use, unused_imports, unstable)]

#[macro_use] extern crate log;

use factory::{Record, RecordResult};
use std::string::ToString;
use std::old_io::IoError;

pub mod controller;
pub mod factory;
mod utils;

struct Txtdb {
    factory: factory::Factory,
}

impl Txtdb {

    pub fn new(factory: factory::Factory) -> Txtdb {
        Txtdb {
            factory: factory,
        }
    }

    /// Add a new record to the database. Returns the id of the record added.
    #[allow(dead_code, unused_must_use, unused_variables)]
    fn add<T: ToString>(&mut self, record: T) -> RecordResult<u64, String> {
        Ok(1)
    }

    /// Removes a record with the id provided if it exists.
    /// Returns a `RecordResult` of the record removed.
    #[allow(dead_code, unused_must_use, unused_variables)]
    fn remove_id(&mut self, id: u64) -> RecordResult<Record, String> {
        Err("Not implemented yet".to_string())
    }

    /// Finds and removes the first instance of a record that matches the one provided.
    /// Returns the id of the record it removes.
    #[allow(dead_code, unused_must_use, unused_variables)]
    fn remove(&mut self, record: Record) -> RecordResult<u64, String> {
        Err("Not implemented yet".to_string())
    }

    /// Searches for a record with the id provided.
    /// Returns a copy of the record.
    #[allow(dead_code, unused_must_use, unused_variables)]
    fn find_id(id: u64) -> RecordResult<Record, String> {
        // 1. Read each line?
        // 2. Check if the ID matches
        // 3. Return
        Err("Not implemented yet".to_string())
    }

    /// Searches for the first instance of a record that matches the one provided.
    /// Returns the id of the record in the database.
    #[allow(dead_code, unused_must_use, unused_variables)]
    fn find(&self, record: Record) -> RecordResult<u64, String> {
        // TODO, how do you create a `Record` if you don't know the id?
        //  Since we aren't using it, should we document not having the id in there?
        //
        // 1. Base64 encode the Record
        // 2. Read each line to find the match encoded value
        // 3. Return id
        Err("Not implemented yet".to_string())
    }
}

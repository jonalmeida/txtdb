//#![allow(dead_code, unused_must_use, unused_imports, unstable)]

// Temporary warning removal until old_io is updated et al.
#![feature(io, collections, core)]


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

    #[allow(dead_code, unused_must_use, unused_variables)]
    fn add<T: ToString>(&mut self, record: T) -> RecordResult<u64, String> {
        //! Add a new record to the database. Returns the id of the record added.
        Ok(1)
    }

    #[allow(dead_code, unused_must_use, unused_variables)]
    fn remove_id(&mut self, id: u64) -> RecordResult<Record, String> {
        //! Removes a record with the id provided if it exists.
        //! Returns a `RecordResult` of the record removed.
        Err("Not implemented yet".to_string())
    }

    #[allow(dead_code, unused_must_use, unused_variables)]
    fn remove(&mut self, record: Record) -> RecordResult<u64, String> {
        //! Finds and removes the first instance of a record that matches the one provided.
        //! Returns the id of the record it removes.
        Err("Not implemented yet".to_string())
    }

    #[allow(dead_code, unused_must_use, unused_variables)]
    fn find_id(id: u64) -> RecordResult<Record, String> {
        //! Searches for a record with the id provided.
        //! Returns a copy of the record.

        // 1. Read each line?
        // 2. Check if the ID matches
        // 3. Return
        Err("Not implemented yet".to_string())
    }

    #[allow(dead_code, unused_must_use, unused_variables)]
    fn find(&self, record: Record) -> RecordResult<u64, String> {
        //! Searches for the first instance of a record that matches the one provided.
        //! Returns the id of the record in the database.

        // TODO, how do you create a `Record` if you don't know the id?
        //  Since we aren't using it, should we document not having the id in there?
        //
        // 1. Base64 encode the Record
        // 2. Read each line to find the match encoded value
        // 3. Return id
        Err("Not implemented yet".to_string())
    }
}

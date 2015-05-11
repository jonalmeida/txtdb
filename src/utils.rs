#![allow(dead_code, unused_must_use, unused_imports, unstable)]
// Any utility functions that are misc can go here

extern crate log;
extern crate rustc_serialize;

use self::rustc_serialize::base64::{STANDARD, FromBase64, ToBase64};

pub fn string_slice(original: String) -> Vec<String> {
    let vec_of_str: Vec<&str> = (&*original).split(' ').collect();
    let mut vec_of_string: Vec<String> = Vec::new();
    for x in vec_of_str.iter() {
        vec_of_string.push(x.to_string());
    }
    return vec_of_string;
}

// Encodes a record to Base64 (with standard encoding).
pub fn encode_record(item: String) -> String {
    item.container_as_bytes().to_base64(STANDARD)
}

// Decoes a record from Base64.
//TODO Maybe change this to JSON later?
pub fn decode_record(item: String) -> String {
    match (&*item).from_base64() {
        Ok(vec) => {
            match String::from_utf8(vec) {
                Ok(string)  => string,
                Err(..)     => {
                    error!("Invalid UTF-8 sequence.");
                    "".to_string()
                },
            }
        }
        Err(..) => {
            error!("Corrupt data, unable to decode.");
            "".to_string()
        },
    }
}

#[test]
fn test_string_slice() {
    let expected = vec!["This".to_string(), "is".to_string(),
                        "a".to_string(), "line".to_string()];
    let input = String::from_str("This is a line");
    let output = string_slice(input);

    assert_eq!(expected, output);
}

#[test]
fn test_encode_string() {
    use utils;
    let expected = "dGhpcyBpcyBhIGxpbmU=";
    let result = utils::encode_record("this is a line".to_string());
    assert_eq![expected, result];
}

#[test]
fn test_decode_string() {
    use utils;
    let expected = "this is a line";
    let result = utils::decode_record("dGhpcyBpcyBhIGxpbmU=".to_string());
    assert_eq![expected, result];
}

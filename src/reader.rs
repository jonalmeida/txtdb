extern crate serialize;

use std::io::{File, Open, Append, Read, ReadWrite};
use std::io::fs::PathExtensions;
use std::io::{BufferedReader, BufferedWriter};
use std::path::BytesContainer;
use self::serialize::base64::{STANDARD, FromBase64, ToBase64};

pub type ReaderResult<T, E> = Result<T, E>;

pub struct Reader {

    path: Path,

    read_buffer: BufferedReader<File>,

    write_buffer: BufferedWriter<File>,

}

struct Record {

    id: u64,

    payload: String,

    metadata: String,

}

pub trait ReaderFile {

    fn open(&self) -> Box<File>;

    fn insert_string(&mut self, String);

    fn encode_record(&self, String) -> String;

    fn decode_record(&self, String) -> String; //TODO Maybe change this to JSON later?

}

impl Reader {

    pub fn new(apath: Path) -> Reader {
        Reader {
            path: {
                if !apath.exists() { File::create(&apath.clone()); }
                apath.clone()
            },
            read_buffer: {
                match File::open_mode(&apath.clone(), Open, Read) {
                    Ok(file)    => { BufferedReader::new(file) },
                    Err(..)     => { panic!("Failed to create a read buffer to file path."); }
                }
            },
            write_buffer: {
                match File::open_mode(&apath.clone(), Append, ReadWrite) {
                    Ok(file)    => { BufferedWriter::new(file) },
                    Err(..)     => { panic!("Failed to create a write buffer to file path."); }
                }
            }
        }
    }

    fn spill(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for line_iter in self.read_buffer.lines() {
        //    println!("{}", line_iter.unwrap());
        //    let line = String::from_str(file.read_line().ok().expect("Nothing to read.").trim());
            result.push(line_iter.unwrap().trim().to_string());
        }
        return result;
    }

    fn insert_str(&mut self, item: &str) {
        self.write_buffer.write_line(item);
        self.write_buffer.flush();
    }

    fn insert(&mut self, item: &[u8]) {
        self.write_buffer.write(item);
        self.write_buffer.flush();
    }

}

impl ReaderFile for Reader {

    fn open(&self) -> Box<File> {
        match File::open_mode(&self.path, Open, ReadWrite) {
            Ok(file)    => box file,
            Err(..)     => { panic!("File {} couldn't be opened!", &self.path.display()); },
        }
    }

    fn insert_string(&mut self, item: String) {
        self.insert_str(item.as_slice());
    }

    fn encode_record(&self, item: String) -> String {
        item.container_as_bytes().to_base64(STANDARD)
    }

    fn decode_record(&self, item: String) -> String {
        match item.as_slice().from_base64() {
            Ok(vec) => {
                match String::from_utf8(vec) {
                    Ok(e)       => e,
                    Err(..)  => panic!("Invalid UTF-8 sequence."),
                }
            }
            Err(..) => panic!("Corrupt data, unable to decode."),
        }
    }

}

#[test]
fn test_open_file() {
    let reader = Reader::new(Path::new("tests/base-test.txt"));
}

#[test]
fn test_create_file() {
    use std::io::fs;
    use std::io::fs::PathExtensions;
    let path = Path::new("tests/base-test-created.txt");
    assert!(!path.exists());
    let reader = Reader::new(path.clone());
    assert!(path.exists());
}

#[test]
fn test_read_file() {
    // We should output the entire contents of the database file we open
    // into standard output.
    let mut reader = setup();
    let expected = vec!["10 11".to_string(), "20 21".to_string()];
    assert_eq!(expected, reader.spill());
}

#[test]
fn test_write_string_to_file() {
    let mut reader = setup();
    let expected = vec!["10 11".to_string(), "20 21".to_string(), "30 31".to_string()];
    reader.insert_string("30 31".to_string());
    assert_eq![expected, reader.spill()];
}

#[test]
fn test_write_str_to_file() {
    let mut reader = setup();
    let expected = vec!["10 11".to_string(), "20 21".to_string(), "30 31".to_string()];
    reader.insert_str("30 31");
    assert_eq![expected, reader.spill()];
}

#[test]
fn test_encode_string() {
    let reader = setup();
    let expected = "dGhpcyBpcyBhIGxpbmU=";
    let result = reader.encode_record("this is a line".to_string());
    assert_eq![expected, result];
}

#[test]
fn test_decode_string() {
    let reader = setup();
    let expected = "this is a line";
    let result = reader.decode_record("dGhpcyBpcyBhIGxpbmU=".to_string());
    assert_eq![expected, result];
}

#[allow(dead_code, unused_must_use)]
fn setup() -> Reader {
    use std::io::fs;

    let mut file = File::create(&Path::new("tests/base-test.txt")).ok().expect("fooo");
    file.write_str("10 11\n20 21\n");

    let p = Path::new("tests/base-test-created.txt");
    if p.exists() {
        fs::unlink(&p);
    }

    Reader::new(Path::new("tests/base-test.txt"))
}

extern crate serialize;

use std::io::{File, Open, Append, Read, ReadWrite};
use std::io::fs::PathExtensions;
use std::io::{BufferedReader, BufferedWriter};
use std::path::BytesContainer;
use self::serialize::base64::{STANDARD, FromBase64, ToBase64};

// A result type that's specfici to the Reader module.
// TODO Decide if this is necessary
pub type ReaderResult<T, E> = Result<T, E>;

// Reader struct of its basic properties.
pub struct Reader {
    // Path to file where the Reader is created.
    path: Path,
    // BufferedReader for reading the file. Initialized with the Path.
    read_buffer: BufferedReader<File>,
    // BufferedWriter for writing to the file. Initialized with the Path.
    write_buffer: BufferedWriter<File>,
}

// ReaderFile traits
pub trait ReaderFile {
    // Opens a new File to the Path provided.
    // Returns a boxed File.
    fn open(&self) -> Box<File>;
    // Inserts a string to the database.
    fn insert_string(&mut self, String);
    // Encodes a record to Base64 (with standard encoding).
    fn encode_record(&self, String) -> String;
    // Decoes a record from Base64.
    fn decode_record(&self, String) -> String; //TODO Maybe change this to JSON later?

}

impl Reader {
    // Creates a new Reader from the Path provided.
    // Opens a new BufferedReader and BufferedWriter (with Append mode) to the file.
    // If the file doesn't exist, it is created.
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

    // This is a helder function that realistically shouldn't exist in production.
    // Used primarily for "spilling" the entire database file into a Vec<String>
    fn spill(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for line_iter in self.read_buffer.lines() {
        //    println!("{}", line_iter.unwrap());
        //    let line = String::from_str(file.read_line().ok().expect("Nothing to read.").trim());
            result.push(line_iter.unwrap().trim().to_string());
        }
        return result;
    }

    // Inserts a &str into the database.
    #[warn(experimental)]
    fn insert_str(&mut self, item: &str) {
        self.write_buffer.write_line(item);
        self.write_buffer.flush();
    }

    // Inserts a byte array into the database.
    #[warn(experimental)]
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

// Test setup code. Current functions:
//  - Create a new file 'tests/base-test.txt'
//  - Write a 2x2 matrix of records into the base-test.txt file
//  - If 'tests/base-test-created.txt' is created, delete it
//  - Returns a Reader object to 'tests/base-test.txt'
#[allow(dead_code, unused_must_use)]
fn setup() -> Reader {
    use std::io::fs;

    let mut file = File::create(&Path::new("tests/base-test.txt")).ok().expect("Unable to create test file");
    file.write_str("10 11\n20 21\n");

    let p = Path::new("tests/base-test-created.txt");
    if p.exists() {
        fs::unlink(&p);
    }

    Reader::new(Path::new("tests/base-test.txt"))
}

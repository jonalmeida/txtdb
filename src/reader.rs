use std::io::{File, Open, Append, Read, ReadWrite};
use std::io::{BufferedReader, BufferedWriter};

pub type ReaderResult<T, E> = Result<T, E>;

pub struct Reader {
    path: Path,

    read_buffer: BufferedReader<File>,

    write_buffer: BufferedWriter<File>,
}

pub trait ReaderFile {

    fn create(&self) -> Box<File>;

    fn open(&self) -> Box<File>;

    fn insert_string(&mut self, String);

    fn insert_str(&mut self, &str);

    fn insert(&mut self, item: &[u8]);

    fn spill(&mut self) -> Vec<String>;
}

impl Reader {
    pub fn new(apath: Path) -> Reader {
        Reader {
            path: apath.clone(),
            read_buffer: {
                let file = File::open_mode(&apath.clone(), Open, Read).ok().expect("ERRRRRRR");
                BufferedReader::new(file)
            },
            write_buffer: {
                let file = File::open_mode(&apath.clone(), Open, ReadWrite).ok().expect("Errrr");
                BufferedWriter::new(file)
            }
        }
    }
}

impl ReaderFile for Reader {

    fn create(&self) -> Box<File> {
        match File::create(&self.path) {
            Ok(file)    => box file,
            Err(..)     => { panic!("Unable to create file at {}", &self.path.display()); },
        }
    }

    fn open(&self) -> Box<File> {
        match File::open_mode(&self.path, Open, ReadWrite) {
            Ok(file)    => box file,
            Err(..)     => { panic!("File {} couldn't be opened!", &self.path.display()); },
        }
    }

    fn insert_string(&mut self, item: String) {
        self.insert_str(item.as_slice());
    }

    fn insert_str(&mut self, item: &str) {
        //self.insert(item.as_bytes());
        self.write_buffer.write_line(item);
        self.write_buffer.flush();
    }

    fn insert(&mut self, item: &[u8]) {
        self.write_buffer.write(item);
        self.write_buffer.flush();
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
}

#[test]
fn test_open_file() {
    let reader = Reader::new(Path::new("tests/base-test.txt"));
}

// We should output the entire contents of the database file we open
// into standard output.
#[test]
fn test_read_file() {
    setup();
    let expected = vec!["10 11".to_string(), "20 21".to_string()];
    let mut reader = Reader::new(Path::new("tests/base-test.txt"));
    assert_eq!(expected, reader.spill());
}

#[test]
fn test_write_string_to_file() {
    setup();
    let expected = vec!["My new line".to_string()];
    let mut reader = Reader::new(Path::new("tests/base-test.txt"));
    reader.insert_str("My new line");
    assert_eq![expected, reader.spill()];
    setup(); // Temporary until this test is not the last test
}

#[test]
fn test_write_str_to_file() {
    setup();

}

#[allow(dead_code)]
fn setup() {
    let mut reader = Reader::new(Path::new("tests/base-test.txt"));
    reader.insert_str("10 11\n20 21\n");
}

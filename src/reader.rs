use std::io::{File, Open, ReadWrite};
use std::io::BufferedStream;

pub type ReaderResult<T, E> = Result<T, E>;

pub struct Reader {
    //TODO Share one BufferedStream in the Reader to avoid overlapping actions
    path: Path,
}

pub trait ReaderFile {

    fn create(&self) -> Box<File>;

    fn open(&self) -> Box<File>;

    fn insert_string(&self, String);

    fn insert_str(&self, &str);

    fn insert(&self, item: &[u8]);

    fn spill(&self) -> Vec<String>;
}

impl Reader {
    pub fn new(path: Path) -> Reader {
        Reader {
            path: path,
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

    fn insert_string(&self, item: String) {
        self.insert_str(item.as_slice());
    }

    fn insert_str(&self, item: &str) {
        self.insert(item.as_bytes());
    }

    fn insert(&self, item: &[u8]) {
        self.open().write(item);
    }

    fn spill(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut file = BufferedStream::new(*self.open());
        for line_iter in file.lines() {
        //    println!("{}", line_iter.unwrap());
            //let line = String::from_str(file.read_line().ok().expect("Nothing to read.").trim());
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
    let reader = Reader::new(Path::new("tests/base-test.txt"));
    assert_eq!(expected, reader.spill());
}

#[test]
fn test_write_file() {
    setup();
    let expected = vec!["My new line".to_string()];
    let mut reader = Reader::new(Path::new("tests/base-test.txt"));
    reader.insert_str("My new line");
    assert_eq![expected, reader.spill()];
    setup(); // Temporary until this test is not the last test
}

fn setup() {
    let reader = Reader::new(Path::new("tests/base-test.txt"));
    let file_reader = reader.create();
    reader.insert_str("10 11\n20 21");
}

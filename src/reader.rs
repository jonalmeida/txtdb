use std::io::File;
use std::io::IoResult;
use std::io::BufferedReader;

pub type ReaderResult<T, E> = Result<T, E>;

pub struct Reader {
    path: Path,
}

pub trait ReaderFile {

    fn create(&self) -> File;

    fn open(&self) -> File;

    fn insert(&self, String);

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
    fn create(&self) -> File {
        match File::create(&self.path) {
            Ok(file)    => file,
            Err(..)     => { panic!("Unable to create file at {}", &self.path.display()); },
        }
    }

    fn open(&self) -> File {
        match File::open(&self.path) {
            Ok(file)    => { file },
            Err(..)     => { panic!("File {} couldn't be opened!", &self.path.display()); },
        }
    }

    fn insert(&self, item: String) {
        //&self.file.write(item);
        &self.open().write_str(item.as_slice());
    }

    fn spill(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut file = BufferedReader::new(self.open());
        //for line_iter in file.lines() {
        //    println!("{}", line_iter.unwrap());
            let line = String::from_str(file.read_line().ok().expect("Nothing to read.").trim());
            result.push(line);
        //}
        result
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
    let expected = vec![String::from_str("10 11")];
    let reader = Reader::new(Path::new("tests/base-test.txt"));
    assert_eq!(expected, reader.spill());
}

#[test]
fn test_write_file() {
    let mut reader = Reader::new(Path::new("tests/base-test.txt"));
    reader.insert("My new line".to_string());
}

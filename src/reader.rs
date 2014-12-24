use std::io::File;
use std::io::BufferedReader;

pub type ReaderResult<T, E> = Result<T, E>;

pub struct Reader {
    path: Path,
}

pub trait ReaderFile {
    fn create(&self);

    fn open(&self) -> File;

    //fn insert(&self, String);

    fn spill(&self);
}

impl Reader {
    pub fn new(path: Path) -> Reader {
        Reader {
            path: path,
        }
    }
}

impl ReaderFile for Reader {
    fn create(&self) {
        File::create(&self.path);
    }

    fn open(&self) -> File {
        let file = File::open(&self.path);
        match file {
            Ok(file)    => { file },
            Err(..)     => { panic!("File couldn't be opened!"); },
        }
    }

    //fn insert(&self, item: String) {
    //    self.file.write(item);
    //}

    fn spill(&self) {
        let mut file = BufferedReader::new(self.open());
        for line_iter in file.lines() {
            println!("{}", line_iter.unwrap());
        }
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
    let reader = Reader::new(Path::new("tests/base-test.txt"));
    reader.spill();
}

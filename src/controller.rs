extern crate serialize;

use std::io::{File, Open, Append, Read, ReadWrite};
use std::io::TempDir;
use std::io::fs::PathExtensions;
use std::io::{BufferedReader, BufferedWriter};
use std::path::BytesContainer;
use self::serialize::base64::{STANDARD, FromBase64, ToBase64};

/// A result type that's specfici to the Reader module.
/// TODO Decide if this is necessary
pub type ReaderResult<T, E> = Result<T, E>;

/// Reader struct of its basic properties.
pub struct Reader {
    /// Path to file where the Reader is created.
    path: Path,
    /// BufferedReader for reading the file. Initialized with the Path.
    read_buffer: BufferedReader<File>,
    /// BufferedWriter for writing to the file. Initialized with the Path.
    write_buffer: BufferedWriter<File>,
}

/// ReaderFile traits
pub trait ReaderFile {
    // Opens a new File to the Path provided.
    // Returns a boxed File.
    fn open(&self) -> Box<File>;
    // Inserts a string to the database.
    fn insert_string(&mut self, String);
}

impl Reader {
    /// Creates a new Reader from the Path provided.
    /// Opens a new BufferedReader and BufferedWriter (with Append mode) to the file.
    /// If the file doesn't exist, it is created.
    // TODO, create a .lock file to let other readers know the database is in use (see: #2).
    pub fn new(apath: &Path) -> Reader {
        Reader {
            path: {
                if !apath.exists() { File::create(&apath.clone()); }
                apath.clone()
            },
            read_buffer: {
                match File::open_mode(&apath.clone(), Open, Read) {
                    Ok(file)    => { BufferedReader::new(file) },
                    Err(..)     => {
                        panic!("Failed to create a read buffer to file path: {}", apath.display())
                    }
                }
            },
            write_buffer: {
                match File::open_mode(&apath.clone(), Append, ReadWrite) {
                    Ok(file)    => { BufferedWriter::new(file) },
                    Err(..)     => {
                        panic!("Failed to create a write buffer to file path: {}", apath.display())
                    }
                }
            }
        }
    }

    /// This is a helder function that realistically shouldn't exist in production.
    /// Used primarily for "spilling" the entire database file into a Vec<String>
    fn spill(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for line_iter in self.read_buffer.lines() {
            result.push(line_iter.unwrap().trim().to_string());
        }
        return result;
    }

    /// Inserts a &str into the database.
    fn insert_str(&mut self, item: &str) {
        self.write_buffer.write_line(item);
        self.write_buffer.flush();
    }

    /// Inserts a byte array into the database.
    fn insert(&mut self, item: &[u8]) {
        self.write_buffer.write(item);
        self.write_buffer.flush();
    }

    /// Read a &str from the database
    fn read_line(&mut self) -> String {
        match self.read_buffer.read_line() {
            Ok(string)  => { string.to_string() },
            Err(..)     => { panic!("Unable to read next line. BufferedReader error."); },
        }
    }

    fn file_lock_created(&self) -> bool {
        use std::str;
        let mut filelock_path = self.path.clone();

        // Surely, there's a less ugly way to take the filename of a Path and convert it to a string?!
        let mut filename_lock: String = str::from_utf8(self.path.filename().unwrap()).unwrap().to_string();
        filename_lock.push_str(".lock");
        filelock_path = filelock_path.join(filename_lock);

        match File::create(&filelock_path) {
            Ok(..)  => true,
            Err(..) => false,
        }
    }

}

impl ReaderFile for Reader {

    fn open(&self) -> Box<File> {
        match File::open_mode(&self.path, Open, ReadWrite) {
            Ok(file)    => Box::new(file),
            Err(..)     => { panic!("File {} couldn't be opened!", &self.path.display()); },
        }
    }

    fn insert_string(&mut self, item: String) {
        self.insert_str(item.as_slice());
    }

}

#[test]
fn test_open_file() {
    let reader = Reader::new(&Path::new("tests/base-test.txt"));
}

#[test]
fn test_create_file() {
    use std::io::fs;
    use std::io::fs::PathExtensions;
    use std::rand;
    let mut path_str = String::from_str("tests/");
    path_str.push_str(rand::random::<usize>().to_string().as_slice());
    path_str.push_str(".txt");
    let path = Path::new(path_str);
    assert!(!path.exists());
    let reader = Reader::new(&path.clone());
    assert!(path.exists());
    fs::unlink(&path);
}

#[test]
fn test_read_file() {
    // We should output the entire contents of the database file we open
    // into standard output.
    let (tempdir, path) = setup();
    let mut reader = Reader::new(&path);
    let expected = vec!["10 11".to_string(), "20 21".to_string()];
    assert_eq!(expected, reader.spill());
}

#[test]
fn test_write_string_to_file() {
    let (tempdir, path) = setup();
    let mut reader = Reader::new(&path);
    let expected = vec!["10 11".to_string(), "20 21".to_string(), "30 31".to_string()];
    reader.insert_string("30 31".to_string());
    assert_eq![expected, reader.spill()];
}

#[test]
fn test_write_str_to_file() {
    let (tempdir, path) = setup();
    let mut reader = Reader::new(&path);
    let expected = vec!["10 11".to_string(), "20 21".to_string(), "30 31".to_string()];
    reader.insert_str("30 31");
    assert_eq![expected, reader.spill()];
}

#[test]
fn test_file_path_lock() {
    use std::str;

    let (tempdir, path) = setup();
    let mut expected = path.clone();
    expected.pop();

    // Surely, there's a less ugly way to take the filename of a Path and convert it to a string?!
    let mut filename_lock: String = str::from_utf8(path.filename().unwrap()).unwrap().to_string();
    filename_lock.push_str(".lock");
    let expected_filelock_path = expected.join(filename_lock);

    let reader = Reader::new(&path);

    assert!(expected_filelock_path.exists() && expected_filelock_path.is_file());
}

/// Test setup code. Current functions:
/// - Create a new file with `TempDir` and a random name.
/// - Write a 2x2 matrix of records into the base-test.txt file
/// - Returns a tuple of `TempDir` and `Path` to the file.
/// - The path is for r/w access and `TempDir` is so that the directory
///   isn't deleted before the test is completed.
#[allow(dead_code, unused_must_use)]
fn setup() -> (TempDir, Path) {
    use std::rand;
    use std::io::fs;
    use std::io::TempDir;

    let tmpdir = match TempDir::new("txtdb-tests") {
        Ok(dir) => dir,
        Err(..) => panic!("Unable to create test dir in temporary directory."),
    };

    let final_dir = tmpdir.path().join(rand::random::<usize>().to_string());

    let mut file = File::create(&final_dir.clone());
    file.write_str("10 11\n20 21\n");

    (tmpdir, final_dir)
}

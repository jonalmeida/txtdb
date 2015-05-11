#![allow(dead_code, unused_must_use, unused_imports, unstable)]

extern crate rustc_serialize;
extern crate log;

// Old imports, refine last
use utils;
use std::fmt;
use std::str;
use std::string;
use std::ops::Drop;
use self::rustc_serialize::base64::{STANDARD, FromBase64, ToBase64};
// New imports for Rust 1.0
use std::path::Path;
use std::io::{BufWriter, BufReader};
use std::fs::{File, PathExt};

/// A result type that's specfici to the Reader module.
/// TODO Decide if this is necessary
pub type ReaderResult<T, E> = Result<T, E>;

/// Reader struct of its basic properties.
pub struct Reader {
    /// Path to file where the Reader is created.
    path: Path,
    /// BufferedReader for reading the file. Initialized with the Path.
    read_buffer: BufReader<File>,
    /// BufferedWriter for writing to the file. Initialized with the Path.
    write_buffer: BufWriter<File>,
    /// Index counter to know how many records exist.
    id_count: u64,
}

/// ReaderFile traits
pub trait ReaderFile {
    // Opens a new File to the Path provided.
    // Returns a boxed File.
    fn open(&self) -> File;
    // Inserts a string to the database.
    fn insert_string(&mut self, String);
}

impl fmt::Debug for Reader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reader: ( path: {} )", self.path.display())
    }
}

impl ToString for Reader {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Reader {
    /// Creates a new Reader from the Path provided.
    /// Opens a new BufReader and BufWriter (with Append mode) to the file.
    /// If the file doesn't exist, it is created.
    // TODO: create a .lock file to let other readers know the database is in use (see: #2).
    pub fn new(path: &Path) -> Reader {
        Reader::file_lock_create(path);
        // if file_lock exists, panic and crash with appropriate error.
        // Error: A .lock file already exists, if this is from a previous session, consider
        // deleting the .lock file.

        // Check if file exists or not. If not, create it.
        if path.is_file() {
            File::create(&path);
        }
        // Create a buffer_writer and buffer_reader.
        let buffer_reader = BufReader::new(&path);
        let buffer_writer = BufWriter::new(&path);
        // Get the current_record count or set it to zero. See old_controller for logic.
        Reader {
            path: path.clone(),
            read_buffer: buffer_reader,
            write_buffer: buffer_writer,
            id_count: current_record_count,
        }
    }

    /// This is a helder function that realistically shouldn't exist in production.
    /// Used primarily for "spilling" the entire database file into a Vec<String>
    fn spill(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut buffer_reader = BufReader::new(File::open(&self.path.clone()));
        for line_iter in buffer_reader.lines() {
            result.push(line_iter.unwrap().trim().to_string());
        }
        return result;
    }

    /// Inserts a &str into the database.
    fn insert_str(&mut self, item: &str) {
        self.write_buffer.write_line(item);
        self.write_buffer.flush();
        self.id_count = self.id_count + 1;
        self.update_counter(self.id_count);
    }

    /// Inserts a byte array into the database.
    fn insert(&mut self, item: &[u8]) {
        self.write_buffer.write(item);
        self.write_buffer.flush();
        self.id_count = self.id_count + 1;
        self.update_counter(self.id_count);
    }

    /// Read a &str from the database
    fn read_line(&mut self) -> String {
        match self.read_buffer.read_line() {
            Ok(string)  => { string.to_string() },
            Err(..)     => {
                error!("Unable to read next line. BufReader error.");
                "".to_string()
            },
        }
    }

    /// Creates a .lock file to let other processes know that the database is in use.
    /// This is still unfinished and should be considered broken.
    fn file_lock_create(lockpath: &Path) -> Path {
        // Check if lock path exists, if not return false and create a new lock file
        if lockpath.exists() {
            return lockpath
        }

        // Create a filename.lock
    }

    /// Removes .lock file when the reader process is completed.
    fn file_lock_remove(&self, filelock: &Path) -> bool {
        fs::remove_file(&filelock.clone());
        filelock.exists()
    }

    /// Updates database counter on disk.
    fn update_counter(&self, value: u64) {
        // Get file to open for writing
        let mut buffer_writer = BufWriter::new(file);
        buffer_writer.write_line(value.to_string().as_slice());
        buffer_writer.flush();
    }

}

impl Drop for Reader {
    fn drop(&mut self) {
        // Remove the lock file.
    }
}

impl ReaderFile for Reader {

    fn open(&self) -> File {
        // Open file
    }

    fn insert_string(&mut self, item: String) {
        self.insert_str(&*item);
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_open_file() {
        let reader = Reader::new(&Path::new("tests/base-test.txt"));
    }

    #[test]
    fn test_create_file() {
        use std::rand;
        let mut path_str = String::from_str("tests/");
        path_str.push_str(&*rand::random::<usize>().to_string());
        path_str.push_str(".txt");

        let (tempdir, apath) = setup();
        let path = tempdir.path().join(rand::random::<usize>().to_string());

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
        let expected = vec!["2".to_string(), "10 11".to_string(), "20 21".to_string()];
        assert_eq!(expected, reader.spill());
    }

    #[test]
    fn test_write_string_to_file() {
        let (tempdir, path) = setup();
        let mut reader = Reader::new(&path);
        let expected = vec!["3".to_string(), "10 11".to_string(), "20 21".to_string(), "30 31".to_string()];
        reader.insert_string("30 31".to_string());
        assert_eq![expected, reader.spill()];
    }

    #[test]
    fn test_write_str_to_file() {
        let (tempdir, path) = setup();
        let mut reader = Reader::new(&path);
        let expected = vec!["3".to_string(), "10 11".to_string(), "20 21".to_string(), "30 31".to_string()];
        reader.insert_str("30 31");
        assert_eq![expected, reader.spill()];
    }

    #[test]
    fn test_file_path_lock() {

        let (tempdir, path) = setup();

        let mut expected = path.clone();
        expected.pop();

        // Surely, there's a less ugly way to take the filename of a Path and convert it to a string?!
        let mut filename_lock: String = str::from_utf8(path.filename().unwrap()).unwrap().to_string();
        filename_lock.push_str(".lock");
        expected = expected.join(filename_lock);

        let reader = Reader::new(&expected.clone());
        assert!(expected.exists() && expected.is_file());
    }

    #[test]
    fn test_reader_show() {
        let reader: Reader = Reader::new(&Path::new("tests/file.txt"));
        assert_eq!("Reader: ( path: tests/file.txt )", reader.to_string());
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

        let tmpdir = match TempDir::new("txtdb-tests") {
            Ok(dir) => dir,
            Err(..) => panic!("Cannot create test directory. Tests will fail."),
        };

        let final_dir = tmpdir.path().join(rand::random::<usize>().to_string());

        let mut file = File::create(&final_dir.clone());
        file.write_str("2\n10 11\n20 21\n");

        (tmpdir, final_dir)
    }
}

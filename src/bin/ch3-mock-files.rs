//! Simulate files, one step at a time.

#![allow(dead_code)]

extern crate rand;

use rand::Rng;
use std::fmt;
use std::fmt::{Display};

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_bool(1.0/(n as f64))
}

#[derive(Debug,PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open   => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// Represent a "file", which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    /// Create a new, empty `File`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Create a new `File` with given `name` and `data`.
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    /// Return the file's length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp         = self.data.clone();
        let     read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    } else {
        f.state = FileState::Open;
        return Ok(f);
    }
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    } else {
        f.state = FileState::Closed;
        return Ok(f);
    }
}

fn main() {
    let f6_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let f6 = File::new_with_data("6.txt", &f6_data);

    println!("{:?}", f6);
    println!("{}", f6);
}

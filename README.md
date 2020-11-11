Introducing `vec`
=================

Parameters
----------

Define the parameters.

        let context_lines = 2;
        let needle = "oo";
        let haystack = "Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";

Initialization
--------------

Initialize mutable state variables.

    let mut tags: Vec<usize>                = Vec::new();
    let mut ctx:  Vec<Vec<(usize, String)>> = Vec::new();

First pass
----------

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let neighbors = Vec::with_capacity(2*context_lines + 1);
            ctx.push(neighbors);
        }
    }

    if tags.len() == 0 {
        return;
    }

Second Pass
-----------

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

Output
------

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }

Searching for chspatterns with regular expressions
==================================================

Look outside the standard library for the `regex` crate.

    extern crate regex;

    use regex::Regex;

    fn main() {
        let re = Regex::new("picture").unwrap(); // NOTE: unwrap() is unsafe

        let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

        for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None    => (),
        }
        }
    }

Traversing Arrays
=================

Define some arrays
------------------

Define `one` using an *array literal*. Rust infers its type, probably
`[i32; 3]`.

    let one             = [1, 2, 3];

Explicitly declare `two` to be an array of `3` elements of `u8`.s

    let two: [u8; 3]    = [1, 2, 3];

Repeat `0` `3` times.

    let blank1          = [0; 3];

Explicitly declare the type of a *repeat* expression.

    let blank2: [u8; 3] = [0; 3];

Traverse `arrays`
-----------------

Taking a reference to an array returns a slice, which supports iteration
without needing to call `iter()`.

    for array in &arrays {

    print!("{:?}: ", array);

    for num in array.iter() {
        print!("\t{} + 10 = {}", num, num + 10);
    }

    let mut sum = 0;
    for idx in 0..array.len() {
        sum += array[idx]; // bounds checked
    }

    print!("\t(Σ{:?} = {})", array, sum);
    println!("");

Modeling Files
==============

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

Checking CubeSat Status
=======================

    #[derive(Copy,Clone,Debug)]
    enum StatusMessage {
        Ok,
    }

Ownership
---------

    #[derive(Debug)]
    struct CubeSat {
        id: u64,
    }

    <<enum-status-message>>

    fn check_status(sat: CubeSat) -> CubeSat {
        println!("{:?}: {:?}", sat, StatusMessage::Ok);
        sat
    }

When `check_status` is called, ownership is moved to the local variable
`sat_id`. Then the new `let` binding is "reset".

    fn main() {
        let sat_a = CubeSat { id: 42 };

        let sat_a = check_status(sat_a);

        // "waiting" ...
        let sat_a = check_status(sat_a);
    }

Ground Control to CubeSat
-------------------------

    #[derive(Debug)]
    pub struct Mailbox {
        messages: Vec<Message>,
    }

    #[derive(Debug)]
    pub struct Message {
        to: u64,
        content: String,
    }

    impl Mailbox {
        pub fn new() -> Mailbox {
            Mailbox {
                messages: vec![],
            }
        }

        pub fn post(&mut self, msg: Message) {
            self.messages.push(msg);
        }

        pub fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
            for i in 0..self.messages.len() {
                if self.messages[i].to == recipient.id {
                    let msg = self.messages.remove(i);
                    return Some(msg);
                }
            }

            None
        }

        pub fn clear(&mut self) {
            self.messages.clear();
        }
    }

    pub struct GroundStation {
        mailbox: Mailbox,
        satellites: Vec<CubeSat>,
    }

Using a read-only reference to `self`, taking a mutable borrow of the
`CubeSat` instance `to`, and taking full ownership of the `Message`
instance `msg`, transfer ownership of `msg` into `messages.push()`, and
return the mutated `to`.

    impl GroundStation {
        pub fn new() -> GroundStation {
            GroundStation {
                mailbox: Mailbox::new(),
                satellites: vec![],
            }
        }

        pub fn connect(&mut self, sat_id: u64) -> CubeSat {
            let sat = CubeSat {
                id: sat_id,
            };
            self.satellites.push(sat);
            sat
        }

        pub fn disconnect(&mut self) {
            self.mailbox.clear()
        }

        pub fn send(&mut self, msg: Message) {
            self.mailbox.post(msg);
        }
    }

    #[derive(Copy,Clone,Debug)]
    pub struct CubeSat {
        id: u64,
    }

    impl CubeSat {
        pub fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
            mailbox.deliver(&self)
        }
    }

    fn main() {
        let mut houston = GroundStation::new();

        for sat_id in vec![1,2,3] {
            houston.connect(sat_id);
            let msg = Message { to: sat_id, content: String::from("hello") };
            houston.send(msg);
        }

        for sat in &houston.satellites {
            let msg = &sat.recv(&mut houston.mailbox);
            println!("{:?}: {:?}", sat, msg);
            println!("{:?}", &houston.mailbox);
        }
    }

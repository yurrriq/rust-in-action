# Traversing Arrays

## Define some arrays

Define `one` using an *array literal*. Rust infers its type, probably
`[i32; 3]`.

``` rust
let one             = [1, 2, 3];
```

Explicitly declare `two` to be an array of `3` elements of `u8`.s

``` rust
let two: [u8; 3]    = [1, 2, 3];
```

Repeat `0` `3` times.

``` rust
let blank1          = [0; 3];
```

Explicitly declare the type of a *repeat* expression.

``` rust
let blank2: [u8; 3] = [0; 3];
```

## Traverse `arrays`

Taking a reference to an array returns a slice, which supports iteration
without needing to call `iter()`.

``` rust
for array in &arrays {
```

``` rust
print!("{:?}: ", array);
```

``` rust
for num in array.iter() {
    print!("\t{} + 10 = {}", num, num + 10);
}
```

``` rust
let mut sum = 0;
for idx in 0..array.len() {
    sum += array[idx]; // bounds checked
}
```

``` rust
print!("\t(Σ{:?} = {})", array, sum);
println!("");
```

# Introducing `vec`

## Parameters

Define the parameters.

``` rust
    let context_lines = 2;
    let needle = "oo";
    let haystack = "Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";
```

## Initialization

Initialize mutable state variables.

``` rust
let mut tags: Vec<usize>                = Vec::new();
let mut ctx:  Vec<Vec<(usize, String)>> = Vec::new();
```

## First pass

``` rust
for (i, line) in haystack.lines().enumerate() {
    if line.contains(needle) {
        tags.push(i);

        let neighbors = Vec::with_capacity(2*context_lines + 1);
        ctx.push(neighbors);
    }
}
```

``` rust
if tags.len() == 0 {
    return;
}
```

## Second Pass

``` rust
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
```

## Output

``` rust
for local_ctx in ctx.iter() {
    for &(i, ref line) in local_ctx.iter() {
        let line_num = i + 1;
        println!("{}: {}", line_num, line);
    }
}
```

# Searching for chspatterns with regular expressions

Look outside the standard library for the `regex` crate.

``` rust
extern crate regex;
```

``` rust
use regex::Regex;
```

``` rust
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
```

# Modeling Files

Prevent compliler warnings about unused variables.

``` rust
#![allow(unused_variables)]
```

``` rust
#[derive(Debug)]
```

``` rust
struct File {
    name: String,
    data: Vec<u8>,
}
```

``` rust
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp         = self.data.clone();
        let     read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}
```

``` rust
fn open(f: &mut File) -> bool {
    true
}
```

``` rust
fn close(f: &mut File) -> bool {
    true
}
```

``` rust
fn main() {
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("2.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    open(&mut f3);
    let f3_length = f3.read(&mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text);
}
```

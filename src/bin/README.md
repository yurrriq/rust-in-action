# Introducing `vec`

## Parameters

Define the parameters.

``` rust
    let context_lines = 2;
    let needle = "oo";
    let haystack = "Every face, every shop,
bedroom window, public-house, and dark square is a picture
feverishly turned--in search of what? It is the same with books.
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

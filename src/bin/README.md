# Traversing Arrays

``` rust
fn main() {
```

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

``` rust
let arrays = [one, two, blank1, blank2];
```

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

``` rust
    }
}
```

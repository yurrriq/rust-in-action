fn main() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for candidate in haystack.iter() {
        if candidate == &needle {
            println!("{}", candidate);
            break;
        } else {
            print!(".");
        }
    }
}

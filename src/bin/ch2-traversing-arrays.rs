fn main() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

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
    }
}

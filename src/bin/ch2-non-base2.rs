fn main() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12c;

    println!("{} {} {}", three, thirty, three_hundred);
    println!("{:b} {:b} {:b}", three, thirty, three_hundred);
    println!("{:o} {:o} {:o}", three, thirty, three_hundred);
    println!("{:x} {:x} {:x}", three, thirty, three_hundred);
}

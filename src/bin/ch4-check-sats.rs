#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Copy, Clone, Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat, StatusMessage::Ok);
    sat
}

fn main() {
    let sat_a = CubeSat { id: 42 };

    let sat_a = check_status(sat_a);

    // "waiting" ...
    let sat_a = check_status(sat_a);
}

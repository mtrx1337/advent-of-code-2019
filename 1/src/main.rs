use std::string::String;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    println!("{}", calc(buffer));
}

fn calc(buffer : String) -> i64 {
    let mut sum : i64 = 0;

    for split in buffer.split(',') {
        if ! split.is_empty() {
            let num = split.parse::<i64>().unwrap();
            sum += (f64::floor(num as f64 / 3.0) - 2.0) as i64;
        }
    }

    return sum;
}

use std::string::String;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .unwrap();

    println!("{}", calc(buffer));
}

fn calc(buffer : String) -> i64 {
    let mut sum : i64 = 0;

    for split in buffer.split('\n') {
        if ! split.is_empty() {
            let num = split.parse::<i64>().unwrap();
            sum += num / 3 - 2;
        }
    }

    sum
}

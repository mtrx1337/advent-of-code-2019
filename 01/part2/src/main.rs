use std::string::String;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .unwrap();

    println!("{}", parse_fuel(buffer));
}

fn parse_fuel(buffer : String) -> i64 {
    let mut sum : i64 = 0;

    for split in buffer.split('\n') {
        if ! split.is_empty() {
            let num = split.parse::<i64>().unwrap();
            sum += fuel_recursion(num / 3 - 2);
        }
    }

    return sum;
}

fn fuel_recursion(fuel : i64) -> i64 {
    if fuel > 0 {
        return fuel + fuel_recursion(fuel / 3 - 2);
    }
    return 0;
}

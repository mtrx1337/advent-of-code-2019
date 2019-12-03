use std::string::String;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .unwrap();

    let splits : Vec<&str> = buffer
        .trim()
        .split(',')
        .collect();

    let origin : Vec<i16> = [0, 0].to_vec();
    let mut position = origin.clone();

    for sp in splits {
        let mut direction : char = ' ';

        // set direction
        for c in sp.chars() {
            if ! c.is_digit(10) {
                direction = c;
            }
        }
        let number_str : String = sp.chars()
            .filter(|x| !x.is_digit(10))
            .collect();
        println!("{}", number_str);
        let number : i16 = number_str
            .parse::<i16>()
            .unwrap();

        match direction {
            'U' => {
                position[1] += number;
            }
            'D' => {
                position[1] -= number;
            }
            'L' => {
                position[0] -= number;
            }
            'R' => {
                position[0] += number;
            }
            _ => println!("not a valid direction")
        }
    }
    println!("Distance from {:?} to {:?}", origin, position);
    let distance = i16::abs(position[0] - origin[0]) + i16::abs(position[1]-origin[1]);
    println!("{}", distance);
}

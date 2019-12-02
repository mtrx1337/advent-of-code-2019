use std::string::String;
use std::process::exit;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut splits : Vec<u32> = buffer
        .trim()
        .split(',')
        .map(|x| x
            .parse::<u32>()
            .unwrap())
        .collect();

    splits[1] = 12;
    splits[2] = 2;
    for x in (0..splits.len()).step_by(4) {
        let a = splits[x + 1] as usize;
        let b = splits[x + 2] as usize;
        let c = splits[x + 3] as usize;
        println!("{},{},{},{}", splits[x], a, b, c);
        match splits[x] {
            1 => splits[c] = splits[a] + splits[b],
            2 => splits[c] = splits[a] * splits[b],
            99 => break,
            _ => { println!("Invalid opcode."); exit(1) },
        }
    }

    println!("{}", splits[0]);
}

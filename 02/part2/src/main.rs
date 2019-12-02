use std::string::String;
use std::process::exit;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .unwrap();

    let splits : Vec<u32> = buffer
        .trim()
        .split(',')
        .map(|x| x
             .parse::<u32>()
             .unwrap())
        .collect();

    for p in 0..99 {
        for q in 0..99 {
            let mut sp = splits.clone();
            sp[1] = p;
            sp[2] = q;
            for x in (0..sp.len()).step_by(4) {
                let a = sp[x + 1] as usize;
                let b = sp[x + 2] as usize;
                let c = sp[x + 3] as usize;
                match sp[x] {
                    1 => sp[c] = sp[a] + sp[b],
                    2 => sp[c] = sp[a] * sp[b],
                    _ => break,
                }
            }

            if sp[0] == 19690720 {
                println!("{}", 100 * p + q);
                exit(0);
            }
        }
    }
}

use std::string::String;

fn main() {
    let mut counter : u64 = 0;

    let from = 137683;
    let to = 596253;

    for i in from..to {
        let mut two_in_a_row : bool = false;
        for x in 0..10 {
            // convert integer to character
            let digit : char = x.to_string().chars().next().unwrap();
            let string = i.to_string();
            if decending(&string) {
                continue;
            }
            let count = count_matches(digit, string);
            match count {
                2 => { two_in_a_row = true; break }
                _ => { continue }
            }
        }

        if two_in_a_row {
            counter += 1;
        }
    }

    println!("{}", counter);
}

fn decending(string : &String) -> bool {
    let mut last_char = '0';
    for c in string.chars() {
        if c.to_digit(10) < last_char.to_digit(10) {
            return true;
        }
        last_char = c;
    }
    return false;
}

fn count_matches(digit : char, string : String) -> u8 {
    let amount_of_matches : u8 = string
        .chars()
        .filter(|x| x == &digit)
        .collect::<Vec<char>>()
        .len()
        as u8;

    return amount_of_matches;
}

fn main() {
    let mut counter : u64 = 0;

    let from = 137683;
    let to = 596253;

    for i in from..to {
        let mut has_double_digit : bool = false;
        let mut has_desc_digits : bool = false;
        let mut last_char = ' ';
        for c in i.to_string().chars() {
            if c < last_char {
                has_desc_digits = true;
            }
            if last_char == c {
                has_double_digit = true;
            }
            last_char = c;
        }

        if has_double_digit && !has_desc_digits {
            counter += 1;
        }
    }

    println!("{}", counter);
}

use std::string::String;
use std::io::{self, Read};
use std::collections::LinkedList;

/* TODO:
 X Make list of all the lines
 X intersect every line with one another
 X calculate the distance from the intersection point to the ORIGIN
 X remember which distance was the shortest
 */

const ORIGIN : [i64; 2]= [0, 0];

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

    let mut position : Vec<i64> = ORIGIN.clone().to_vec();

    let mut lines : LinkedList<Vec<Vec<i64>>> = LinkedList::new();
    let mut last_pos : Vec<i64> = ORIGIN.clone().to_vec();

    for sp in splits {
        let mut direction : char = ' ';

        // set direction
        for c in sp.chars() {
            if ! c.is_digit(10) {
                direction = c;
            }
        }

        let number_str : String = sp.chars()
            .filter(|x| x.is_digit(10))
            .collect();

        let number : i64 = number_str
            .parse::<i64>()
            .unwrap();

        match direction {
            'U' => {
                position[1] += number;
            }
            'R' => {
                position[0] += number;
            }
            'D' => {
                position[1] -= number;
            }
            'L' => {
                position[0] -= number;
            }
            _ => {
                println!("{}{} not a valid direction", direction, number);
            }
        }

        let line : Vec<Vec<i64>> = [last_pos.clone(), position.clone()].to_vec();
        lines.push_back(line);
        last_pos = position.clone();
    }

    println!("{}", intersect(lines));
}

fn intersect(lines : LinkedList<Vec<Vec<i64>>>) -> i64 {
    let mut shortest_distance = i64::max_value();
    let comparison_lines = &lines.clone();

    // compare all lines against each other
    for line in lines {
        for comp_line in comparison_lines {
            // A
            let line_a_1 = &line[0];
            // B
            let line_a_2 = &line[1];
            // C
            let line_b_1 = &comp_line[0];
            // D
            let line_b_2 = &comp_line[1];

            // line AB
            let a1 : i64 = line_a_2[1] - line_a_1[1];
            let b1 : i64 = line_a_1[0] - line_a_2[0];
            let c1 : i64 = a1 * line_a_1[0] + b1 * line_a_1[1];

            // line CD
            let a2 : i64 = line_b_2[1] - line_b_1[1];
            let b2 : i64 = line_b_1[0] - line_b_2[0];
            let c2 : i64 = a2 * line_b_1[0] + b2 * line_b_1[1];

            let determinant = a1 * b2 - a2 * b1;

            // calc intersection point
            if determinant == 0 {
                continue;
            } else {
                let x = (b2 * c1 - b1 * c2) / determinant;
                let y = (a1 * c2 - a2 * c1) / determinant;

                let intersection = [x, y].to_vec();

                // calc distance from intersection point to ORIGIN
                let distance = i64::abs(intersection[0] - ORIGIN[0]) + i64::abs(intersection[1] - ORIGIN[1]);

                // is the distance shorter than the last one?
                if distance < shortest_distance {
                    shortest_distance = distance;
                }
            }
        }
    }
    return shortest_distance;
}

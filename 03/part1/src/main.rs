use std::string::String;
use std::io::{self, Read};
use std::collections::LinkedList;

const ORIGIN : [i64; 2]= [0, 0];

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .unwrap();

    let wires : Vec<&str> = buffer
        .split('\n')
        .collect();

    let wire1 : Vec<&str> = wires[0]
        .trim()
        .split(',')
        .collect();

    let wire2 : Vec<&str> = wires[1]
        .trim()
        .split(',')
        .collect();

    let lines_wire1 = create_wire_lines(wire1);
    let lines_wire2 = create_wire_lines(wire2);

    println!("{}", intersect(lines_wire1, lines_wire2));
}

fn create_wire_lines(wire : Vec<&str>) -> LinkedList<Vec<Vec<i64>>> {
    let mut lines : LinkedList<Vec<Vec<i64>>> = LinkedList::new();
    let mut position : Vec<i64> = ORIGIN.clone().to_vec();
    let mut last_pos : Vec<i64> = ORIGIN.clone().to_vec();

    for sp in wire {
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
    return lines;
}

/* TODO */
fn intersect(lines1 : LinkedList<Vec<Vec<i64>>>, lines2 : LinkedList<Vec<Vec<i64>>>) -> i64 {
    let mut shortest_distance = i64::max_value();

    // compare all lines against each other
    for line1 in lines1 {
        for line2 in lines2.clone() {
            let distance = calc_distance(line1.clone(), line2);

            if distance == 0 {
                continue;
            }

            // is the distance shorter than the last one?
            if distance < shortest_distance {
                shortest_distance = distance;
            }
        }
    }
    return shortest_distance;
}

fn calc_distance(line1 : Vec<Vec<i64>>, line2 : Vec<Vec<i64>>) -> i64 {
    // A
    let line_a_1 = &line1[0];
    // B
    let line_a_2 = &line1[1];
    // C
    let line_b_1 = &line2[0];
    // D
    let line_b_2 = &line2[1];

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
    if ! determinant == 0 {
        let x = (b2 * c1 - b1 * c2) / determinant;
        let y = (a1 * c2 - a2 * c1) / determinant;

        let intersection = [x, y].to_vec();

        // calc distance from intersection point to origin
        return i64::abs(intersection[0] - ORIGIN[0]) + i64::abs(intersection[1] - ORIGIN[1]);
    }

    return 0;
}

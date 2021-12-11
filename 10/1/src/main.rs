use std::fs;
use std::collections::HashMap;

// Is this s Dyck joke?

fn main() {
    let bracket_points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let brackets = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let mut points = 0;

    for chunks in contents.lines() {
        let mut stack = vec![];
        for p in chunks.chars() {
            if brackets.contains_key(&p) {
                stack.push(p);
            } else {
                match stack.pop() {
                    Some(q) if *brackets.get(&q).unwrap_or(&' ') == p => continue,
                    Some(_) | None => {
                        points += bracket_points.get(&p).expect("Symbol unknown!");
                        break;
                    }
                }
            }
        }
    }

    println!("Points: {}", points);
}

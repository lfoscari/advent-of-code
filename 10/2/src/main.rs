use std::fs;
use std::collections::HashMap;

// Is this s Dyck joke?

fn main() {
    let bracket_points = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let brackets = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let mut scores = vec![];

    for chunks in contents.lines() {
        let mut stack = vec![];
        let mut invalid = false;

        for p in chunks.chars() {
            if brackets.contains_key(&p) {
                stack.push(p);
            } else {
                match stack.pop() {
                    Some(q) if *brackets.get(&q).unwrap_or(&' ') == p => continue,
                    Some(_) | None => {
                        invalid = true;
                        break
                    }
                }
            }
        }

        if !invalid && !stack.is_empty() {
            stack = stack.iter().rev().map(|b| *brackets.get(b).expect("Symbol unknown")).collect();
            let score = stack.into_iter().fold(0 as i64, |s, b|
                (s * 5) + bracket_points.get(&b).expect("Symbol unknown")
            );
            scores.push(score);
        }
    }

    scores.sort();
    println!("Middle score: {}", scores[scores.len() / 2]);
}

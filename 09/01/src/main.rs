use std::collections::HashSet;
use std::fs;

fn main() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let input = fs::read_to_string("input.txt").expect("Missing input file");

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = vec![tail];

    for instruction in input.lines() {
        let (direction, amount_str) = instruction.split_at(1);
        let amount: u32 = amount_str.strip_prefix(" ").unwrap().parse().unwrap();

        for _ in 0..amount {
            match direction {
                "U" => {
                    head = (head.0, head.1 + 1);

                    if !are_touching(head, tail) {
                        tail = (head.0, tail.1 + 1);
                    }
                },
                "D" => {
                    head = (head.0, head.1 - 1);

                    if !are_touching(head, tail) {
                        tail = (head.0, tail.1 - 1);
                    }
                },
                "R" => {
                    head = (head.0 + 1, head.1);

                    if !are_touching(head, tail) {
                        tail = (tail.0 + 1, head.1);
                    }
                },
                "L" => {
                    head = (head.0 - 1, head.1);

                    if !are_touching(head, tail) {
                        tail = (tail.0 - 1, head.1);
                    }
                },
                _ => panic!("Unknown direction")
            }

            visited.push(tail.clone());
        }
    }

    println!("{:?}", visited.iter().collect::<HashSet<_>>().len());
}

fn are_touching ((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> bool {
    (hx - tx).abs() <= 1 && (hy - ty).abs() <= 1
}

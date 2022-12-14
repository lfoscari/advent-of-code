use std::collections::HashSet;
use std::fs;

const KNOTS: usize = 10;

fn main() {
//     let input = "R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2";

//     let input = "R 5
// U 8
// L 8
// D 3
// R 17
// D 10
// L 25
// U 20";

    let input = fs::read_to_string("input.txt").expect("Missing input file");

    let mut knots = vec![(0, 0); KNOTS];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for instruction in input.lines() {
        let (direction, amount_str) = instruction.split_at(1);
        let amount: u32 = amount_str.strip_prefix(" ").unwrap().parse().unwrap();

        for _ in 0..amount {
            visualize(&knots);

            knots[0] = match (direction, knots[0]) {
                ("U", (hx, hy)) => (hx, hy + 1),
                ("D", (hx, hy)) => (hx, hy - 1),
                ("R", (hx, hy)) => (hx + 1, hy),
                ("L", (hx, hy)) => (hx - 1, hy),
                _ => panic!("Unknown direction")
            };

            for i in 1..KNOTS {
                knots[i] = make_touch(knots[i - 1], knots[i]);
            }

            visited.insert((&knots).last().unwrap().clone());
        }
    }

    println!("{:?}", visited.len());
}

fn make_touch ((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    if (tx - hx).abs() <= 1 && (ty - hy).abs() <= 1 { return (tx, ty) }
    
    if tx == hx && ty > hy { return (hx, hy + 1) }
    if tx == hx && ty < hy { return (hx, hy - 1) }
    if tx > hx && ty == hy { return (hx + 1, hy) }
    if tx < hx && ty == hy { return (hx - 1, hy) }

    // diagonals
    if (hx - tx).abs() == (hy - ty).abs() {
        if tx < hx && ty < hy { return (tx + 1, ty + 1) }
        if tx < hx && ty > hy { return (tx + 1, ty - 1) }
        if tx > hx && ty < hy { return (tx - 1, ty + 1) }
        if tx > hx && ty > hy { return (tx - 1, ty - 1) }
    }

    // horitontal bowtie
    if (hx - tx).abs() > (hy - ty).abs() {
        if tx < hx { return (hx - 1, hy) }
        if tx > hx { return (hx + 1, hy) }
    }

    // vertical bowtie
    if (hx - tx).abs() < (hy - ty).abs() {
        if ty < hy { return (hx, hy - 1) }
        if ty > hy { return (hx, hy + 1) }
    }

    panic!("Impossible");
}

fn visualize(knots: &Vec<(i32, i32)>) {
    let size = 26;
    for y in (0..size).rev() {
        for x in 0..size {
            let mut a_tail = false;
            for (index, &(tx, ty)) in knots.into_iter().enumerate() {
                if tx == x && ty == y {
                    print!("{}", if index == 0 { "H".to_string() } else { index.to_string() });
                    a_tail = true;
                    break;
                }
            }

            if !a_tail {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
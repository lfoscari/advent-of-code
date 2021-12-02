use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");

    let measurements: Vec<i32> = contents
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut count = 0;

    for (i, m) in measurements.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if measurements[i-1] < *m {
            count += 1;
        }
    }

    println!("{}", count);
}

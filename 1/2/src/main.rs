use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");

    let ms: Vec<i32> = contents
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut sums: Vec<i32> = vec![];

    for i in 1..ms.len()-1 {
        sums.push(ms[i - 1] + ms[i] + ms[i + 1]);
    }

    let mut count = 0;

    for (i, s) in sums.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if sums[i-1] < *s {
            count += 1;
        }
    }

    println!("{}", count);
}

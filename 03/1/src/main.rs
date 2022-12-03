use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");

    let ms: Vec<Vec<char>> = contents
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();

    let bit_size = ms[0].len();
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..bit_size {
        let mut freq = 0;

        for m in &ms {
            freq = if m[i] == '1' { freq + 1 } else { freq - 1 };
        }

        let position = ( bit_size - i - 1 ) as u32;

        if freq > 0 {
            gamma += 2_i32.pow(position);
        } else {
            epsilon += 2_i32.pow(position);
        }
    }

    println!("Gamma: {}\nEpsilon: {}\nTotal: {}", gamma, epsilon, gamma * epsilon);
}

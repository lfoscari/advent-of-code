#![feature(drain_filter)]
use std::fs;

fn binary_to_i32(bs: &Vec<char>) -> u32 {
    bs
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x.to_digit(10).unwrap() * 2_u32.pow(i as u32))
        .sum()
}

fn rating_symbol(ms: Vec<Vec<char>>, greater: bool) -> u32 {
    let mut pos = 0;
    let mut numbers = ms;

    while numbers.len() > 1 {
        let main = numbers.drain_filter(|m| m[pos] == '1').collect::<Vec<_>>();

        if greater ^ (main.len() >= numbers.len()) {
            numbers = main;
        }

        pos += 1;
    }

    binary_to_i32(&numbers[0])
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");

    let ms: Vec<Vec<char>> = contents
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();

    let oxygen = rating_symbol(ms.clone(), true);
    let co2 = rating_symbol(ms.clone(), false);

    println!("Oxygen: {}\nCO2 scrubber: {}\nTotal: {}", oxygen, co2, oxygen * co2);
}
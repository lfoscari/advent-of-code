use std::{fs, collections::HashSet}; 

fn main() {
//     let input = "vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw";

    let input = fs::read_to_string("input.txt")
        .expect("Missing input file");

    let mut score = 0;

    for rucksack in input.lines() {
        let objects = rucksack
            .chars()
            .collect::<Vec<char>>();

        let (first, second) = objects
            .split_at(rucksack.len() / 2);

        let unique_first: HashSet<_> = first.into_iter().collect();
        let unique_second: HashSet<_> = second.into_iter().collect();

        let common: HashSet<_> = unique_first
            .intersection(&unique_second)
            .collect();

        score += common
            .into_iter()
            .map(|&c| obj_score(c))
            .sum::<i32>();
    }

    println!("{:?}", score);
}

fn obj_score (o: &char) -> i32 {
    if o.is_lowercase() {
        (*o as i32) - ('a' as i32) + 1
    } else {
        (*o as i32) - ('A' as i32) + 27
    }
}

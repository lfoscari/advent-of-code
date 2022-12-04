use std::{fs, collections::HashSet}; 

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Missing input file");

    let rucksacks: Vec<&str> = input.lines().collect();
    let mut score = 0;

    for group in rucksacks.chunks(3) {
        let mut candidates: HashSet<_> = HashSet::new();

        for rucksack in group {
            let objects: HashSet<_> = rucksack.chars().collect();

            if candidates.len() == 0 {
                candidates = objects
            } else {
                candidates = candidates
                    .intersection(&objects)
                    .map(|&o| o)
                    .collect();
            }
        }

        let badges: Vec<_> = candidates.into_iter().collect();
        assert!(badges.len() == 1, "{:?}", badges);

        score += obj_score(&badges[0]);
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

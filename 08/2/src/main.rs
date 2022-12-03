use std::fs;
use std::collections::*;

// I'm not in the mood, I tried a kinda elegant
// solution, but it did not work, now I'm pissed.

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");

    let digit_patterns: HashMap<_, _> = HashMap::from([
        (0, vec!['a', 'b', 'c', 'e', 'f', 'g']),
        (1, vec!['c', 'f']),
        (2, vec!['a', 'c', 'd', 'e', 'g']),
        (3, vec!['a', 'c', 'd', 'f', 'g']),
        (4, vec!['b', 'c', 'd', 'f']),
        (5, vec!['a', 'b', 'd', 'f', 'g']),
        (6, vec!['a', 'b', 'd', 'e', 'f', 'g']),
        (7, vec!['a', 'c', 'f']),
        (8, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (9, vec!['a', 'b', 'c', 'd', 'f', 'g'])
    ]);

    let segments_digits = HashMap::from([
        // (1, vec![]),
        (2, vec![1]),
        (3, vec![7]),
        (4, vec![4]),
        (5, vec![2, 3, 5]),
        (6, vec![0, 9, 6]),
        (7, vec![8])
    ]);

    let wire_counter = HashMap::from([
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
        ('e', 0),
        ('f', 0),
        ('g', 0),
    ]);

    for display in contents.lines() {  
        let mut segments_wires: HashMap<_, HashMap<_, _>> = HashMap::from([
            ('a', wire_counter.clone()),
            ('b', wire_counter.clone()),
            ('c', wire_counter.clone()),
            ('d', wire_counter.clone()),
            ('e', wire_counter.clone()),
            ('f', wire_counter.clone()),
            ('g', wire_counter.clone())
        ]);
    
        let (patterns, _) = display.split_once(" | ").unwrap();

        for pattern in patterns.split_whitespace() {
            let new_guesses: HashSet<_> = pattern.chars().into_iter().collect();

            for candidate in segments_digits.get(&pattern.len()).unwrap() {
                for segment in digit_patterns.get(candidate).unwrap() {
                    segments_wires.entry(*segment).and_modify(|wire_counter|
                        for wire in &new_guesses {
                            wire_counter.entry(*wire).and_modify(|counter| { *counter += 1 });
                        }
                    );
                }
            }
            println!("Pattern: {:?}\n{:?}\n", pattern, segments_wires);
        }

        break;
    }

    // println!("Total: {}", total);
}
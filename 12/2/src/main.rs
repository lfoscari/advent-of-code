use std::fs;
use std::collections::{ HashMap, HashSet };

fn is_small(cave: &str) -> bool {
    cave.to_lowercase() == cave.to_string()
}

fn count_paths(caves: &HashMap<&str, HashSet<&str>>, path: Vec<&str>, can_revisit_small_cave: bool) -> i32 {
    let mut paths_from_start = 0;
    let from = path.last().unwrap();

    if from == &"end" {
        return 1;
    }

    for cave in caves.get(from).unwrap() {
        if cave == &"start" {
            continue;
        }

        if !path.contains(cave) || !is_small(cave) {
            let mut new_path = path.clone();
            new_path.push(cave);
            paths_from_start += count_paths(caves, new_path, can_revisit_small_cave);
        } else if is_small(cave) && can_revisit_small_cave {
            let mut new_path = path.clone();
            new_path.push(cave);
            paths_from_start += count_paths(caves, new_path, false);
        }
    }

    paths_from_start
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let mut caves: HashMap<&str, HashSet<&str>> = HashMap::new();

    for path in contents.lines() {
        let (a, b) = path.split_once("-").unwrap();
        caves.entry(a).and_modify(|s| { let _ = s.insert(b); }).or_insert(HashSet::from([b]));
        caves.entry(b).and_modify(|s| { let _ = s.insert(a); }).or_insert(HashSet::from([a]));
    }

    let paths = count_paths(&caves, vec!["start"], true);
 
    // println!("Caves: {:?}", caves);
    println!("There are {} paths from start to end", paths);
}
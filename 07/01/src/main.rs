use std::{fs, collections::HashMap};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Missing input file");

    let mut pwd = vec![];
    let mut cursor = 0;
    let terminal: Vec<&str> = input.lines().collect();
    let mut sizes: HashMap<String, u32> = HashMap::new();

    let cd = Regex::new(r"^\$ cd (.*)$").unwrap();
    let ls = Regex::new(r"^\$ ls$").unwrap();

    let file = Regex::new(r"(^[0-9]+) (.*)$").unwrap();
    let dir = Regex::new(r"dir (.*)$").unwrap();

    while cursor < terminal.len() {
        let mut curr = terminal[cursor];

        if let Some(directory) = cd.captures(curr) {
            let dirname = directory.get(1).unwrap().as_str();

            if dirname == ".." {
                pwd.pop();
            } else {
                pwd.push(dirname);
            }

            println!("Now in directory {:?}", pwd);

            cursor += 1;
        } else if ls.is_match(curr) {
            cursor += 1;

            println!("Listing contents of {:?}", pwd);

            while cursor < terminal.len() {
                curr = terminal[cursor];
                if let Some(filedata) = file.captures(curr) {
                    let filesize: u32 = filedata.get(1).unwrap().as_str().parse().unwrap();
                    let filename = filedata.get(2).unwrap().as_str();

                    for index in 0..=pwd.len() {
                        let dir = pwd[0..index].concat();
                        sizes.entry(dir)
                            .and_modify(|s| *s += filesize)
                            .or_insert(filesize);
                    }

                    println!("File {:?}:{:?} in directory {:?}", filename, filesize, pwd);
                    cursor += 1;
                } else if let Some(directory) = dir.captures(curr) {
                    println!("Directory {:?} in {:?}", directory.get(1).unwrap().as_str(), pwd);
                    cursor += 1;
                } else {
                    break;
                }

            }
        } else {
            panic!("Unknown command {:?}", curr);
        }
    }

    println!("Sizes: {:?}", sizes);

    sizes.retain(|_k, &mut v| v <= 100000);
    println!("Result: {:?}", sizes.values().sum::<u32>());
}

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");

    let mut depth = 0;
    let mut position = 0;

    for command in contents.lines() {
        let commands: Vec<&str> = command.split(" ").collect();
        
        let command = commands[0];
        let amount: i32 = commands[1].parse().unwrap();

        match command {
            "forward" => position += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("Unknown command")
        }
    }

    println!("Drpth: {}\nPosition: {}\nCombined: {}", depth, position, depth * position);
}

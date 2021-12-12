use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Ops.");
    let mut unique = 0;

    for display in contents.lines() {
        let (_, output) = display.split_once(" | ").unwrap();

        for d in output.split_whitespace() {
            if [2, 3, 4, 7].contains(&d.len()) {
                unique += 1;
            }
        }
    }

    println!("Unique: {}", unique);
}

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Input file missing");

    let chars = &input.chars().collect::<Vec<char>>();

    for (index, seq) in chars.windows(4).enumerate() {
        let mut vec = seq.to_vec();

        vec.sort();
        vec.dedup();

        if vec.len() == 4 {
            println!("{:?} at {:?}", seq, index + 4);
            return;
        }
    }
}

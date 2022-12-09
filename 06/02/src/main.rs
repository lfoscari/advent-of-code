use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Input file missing");

    let size = 14;
    let chars = &input.chars().collect::<Vec<char>>();

    for (index, seq) in chars.windows(size).enumerate() {
        let mut vec = seq.to_vec();

        vec.sort();
        vec.dedup();

        if vec.len() == size {
            println!("{:?} at {:?}", seq, index + size);
            return;
        }
    }

}

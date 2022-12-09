use std::fs;

fn main() {
//     let input = "    [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2";

    let input = fs::read_to_string("input.txt")
        .expect("Input file missing");

    let columns = 9;
    let mut read_instructions = false;
    let mut lines = input.lines();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; columns];
    let mut instructions: Vec<(usize, usize, usize)> = vec![];

    loop {
        match lines.next() {
            Some (line) if line == "" => read_instructions = true,
            Some (line) if read_instructions => {
                let it = &mut line.chars();
                let mut ns: Vec<usize> = vec![];

                loop {
                    match it.next() {
                        Some(c) if c == ' ' => {
                            let n: String = it.take_while(|&s| s != ' ').collect();
                            ns.push(n.parse().unwrap());
                        },
                        Some(_) => (),
                        None => break
                    }
                }

                instructions.push((ns[0], ns[1] - 1, ns[2] - 1));
            },
            Some (line) => {
                let mut stack = 0;
                let mut index = 0;

                loop {
                    if line.chars().nth(index).unwrap() == '[' {
                        let l = line.chars().nth(index + 1).unwrap();
                        stacks[stack].insert(0, l);
                    }

                    stack += 1;
                    index += 4;
                
                    if index >= 4 * columns {
                        break;
                    } 
                }
            },
            None => break
        }
    }

    for (amount, from, to) in instructions {
        let len = stacks[from].len() - amount;
        let mut to_move = stacks[from].split_off(len);
        to_move.reverse();

        stacks[to].extend(to_move);
    }


    for i in 0..columns {
        print!("{}", stacks[i].last().unwrap());
    }

    println!();

}

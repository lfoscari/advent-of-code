use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Missing input file");

    let mut output = vec![vec![' '; 40]; 6];
    let point = &mut (0, 0);

    let mut cycle = 0;
    let mut register = 1;

    for instruction in input.lines() {
        output = update(output, point, &mut cycle, register);
        match instruction.split_at(4) {
            ("noop", _) => (),
            ("addx", arg) => {
                output = update(output, point, &mut cycle, register);
                register += arg.trim().parse::<i32>().unwrap();
            },
            _ => panic!("Unknown instruction")
        }
    }

    draw(output);
}

fn update(output: Vec<Vec<char>>, point: &mut (usize, usize), cycle: &mut i32, register: i32) -> Vec<Vec<char>> {
    let position = *cycle % 40;
    *cycle += 1;

    let mut refreshed = output;

    if [register - 1, register, register + 1].contains(&position) {
        refreshed[point.0][point.1] = '#';
    } else {
        refreshed[point.0][point.1] = '.';
    }

    if point.1 + 1 == 40 {
        *point = (point.0 + 1, 0);
    } else {
        *point = (point.0, point.1 + 1);
    }

    refreshed
}

fn draw(output: Vec<Vec<char>>) {
    for line in output {
        for content in line {
            print!("{}", content);
        }
        println!();
    }
}

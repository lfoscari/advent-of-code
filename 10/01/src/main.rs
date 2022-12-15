use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Missing input file");

    let mut cycle = 0;
    let mut register = 1;
    let mut measurement = 0;

    for instruction in input.lines() {
        update(&mut measurement, &mut cycle, register);
        match instruction.split_at(4) {
            ("noop", _) => (),
            ("addx", arg) => {
                update(&mut measurement, &mut cycle, register);
                register += arg.trim().parse::<i32>().unwrap();
            },
            _ => panic!("Unknown instruction")
        }
    }

    println!("{:?}", measurement);
}

fn update(measurement: &mut i32, cycle: &mut i32, register: i32) {
    *cycle += 1;
    if (*cycle - 20) % 40 == 0 {
        *measurement += register * *cycle;
    }
}

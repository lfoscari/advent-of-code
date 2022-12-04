use std::fs; 

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Missing input file");

    let mut total = 0;

    for line in input.lines() {
        let moves: Vec<RPS> = line.split(" ").map(move_string).collect();
        println!("{:?} => {:?}", line, moves);
        total += score(moves[0], moves[1]);
    }

    println!("{:?}", total);
}

fn move_string(s: &str) -> RPS {
    match s {
        "A" | "X" => RPS::Rock,
        "B" | "Y" => RPS::Paper,
        "C" | "Z" => RPS::Scissors,
        _ => panic!("Malformed input")
    }
}

fn score (opponent: RPS, me: RPS) -> i32 {
    return match me {
        RPS::Rock => 1 + win(opponent, me),
        RPS::Paper => 2 + win(opponent, me),
        RPS::Scissors => 3 + win(opponent, me)
    }
}

fn win (opponent: RPS, me: RPS) -> i32 {
    return match (opponent, me) {
        (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => 0,
        (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => 3,
        (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => 6
    }
}
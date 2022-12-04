use std::fs; 

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

fn main () {
//     let input = "A Y
// B X
// C Z";

    let input = fs::read_to_string("input.txt")
        .expect("Missing input file");

    let mut total = 0;

    for line in input.lines() {
        let game: Vec<&str> = line.split(" ").collect();

        let opponent = move_string(game[0]);
        let me = outcome_move(opponent, game[1]);

        total += score(opponent, me);
    }

    println!("{:?}", total);
}

fn move_string (s: &str) -> RPS {
    match s {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!("Malformed input")
    }
}

fn outcome_move (mv: RPS, outcome: &str) -> RPS {
    match (mv, outcome) {
        (x, "Y") => x,

        (RPS::Rock, "X") => RPS::Scissors,
        (RPS::Rock, "Z") => RPS::Paper,

        (RPS::Paper, "X") => RPS::Rock,
        (RPS::Paper, "Z") => RPS::Scissors,

        (RPS::Scissors, "X") => RPS::Paper,
        (RPS::Scissors, "Z") => RPS::Rock,

        (_ , _) => panic!("Malformed inputs")
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
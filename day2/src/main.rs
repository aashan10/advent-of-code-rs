use std::fs;
use std::env;
use std::result;



#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}




fn main() {
    let results = read_file();

    let mut score = 0;
    for result in results {
        let points = get_points(result);
        score += points;
    }

    println!("Score: {}", score);
}

fn read_file() -> Vec<(RockPaperScissors, RockPaperScissors)> {
    let file_path = env::current_dir().unwrap();
    let p = file_path.join("src/cheatcode.txt");
    let mut result: Vec<(RockPaperScissors, RockPaperScissors)> = Vec::new();
    

    let contents = fs::read_to_string(p)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let chunks = line.split(" ").collect::<Vec<&str>>();

        let player1 = get_rock_paper_scissors_from_char(chunks[0].chars().nth(0).unwrap());
        let player2 = get_rock_paper_scissors_from_char(chunks[1].chars().nth(0).unwrap());

        result.push((player1, player2));
    }

    result
}


fn get_rock_paper_scissors_from_char(c: char) -> RockPaperScissors {
    match c {
        'A' | 'X' => RockPaperScissors::Rock,
        'B' | 'Y' => RockPaperScissors::Paper,
        'C' | 'Z' => RockPaperScissors::Scissors,
        _ => panic!("Invalid character"),
    }
}

fn get_points((player1, player2): (RockPaperScissors, RockPaperScissors)) -> usize {
    let point = match player2 {
        RockPaperScissors::Rock => 1,
        RockPaperScissors::Paper => 2,
        RockPaperScissors::Scissors => 3,
    };

    let winner_point = match (player1, player2) {
        (RockPaperScissors::Rock, RockPaperScissors::Rock) => 3,
        (RockPaperScissors::Rock, RockPaperScissors::Paper) => 6,
        (RockPaperScissors::Rock, RockPaperScissors::Scissors) => 0,
        (RockPaperScissors::Paper, RockPaperScissors::Rock) => 0,
        (RockPaperScissors::Paper, RockPaperScissors::Paper) => 3,
        (RockPaperScissors::Paper, RockPaperScissors::Scissors) => 6,
        (RockPaperScissors::Scissors, RockPaperScissors::Rock) => 6,
        (RockPaperScissors::Scissors, RockPaperScissors::Paper) => 0,
        (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => 3,
    };

    point + winner_point
}
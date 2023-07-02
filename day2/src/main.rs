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
    let mut score2 = 0;
    for result in results {
        let points = get_points(result);
        let points2 = get_result_point((get_rock_paper_scissors_from_char(result.0), result.1));
        score2 += points2;
        score += points;
    }

    println!("Score: {}", score);
    println!("Score2: {}", score2);
}

fn read_file() -> Vec<(char, char)> {
    let file_path = env::current_dir().unwrap();
    let p = file_path.join("src/cheatcode.txt");
    let mut result: Vec<(char, char)> = Vec::new();
    

    let contents = fs::read_to_string(p)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let chunks = line.split(" ").collect::<Vec<&str>>();

        let player1 = chunks[0].chars().nth(0).unwrap();
        let player2 = chunks[1].chars().nth(0).unwrap();

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


fn get_points((player1, player2): (char, char)) -> usize {
    
    let player1 = get_rock_paper_scissors_from_char(player1);
    let player2 = get_rock_paper_scissors_from_char(player2);

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


fn get_points_if_second_column_is_result_type(results: Vec<(char, char)>) -> usize {

    let mut score = 0;
    for result in results {
        let first = get_rock_paper_scissors_from_char(result.0);

        let second = result.1;

        score += get_result_point((first, second));
    }

    score
}

fn get_result_point((player, result): (RockPaperScissors, char)) -> usize {
    
    let win_point = match result {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };

    let player_move = match (player, result) {
        (RockPaperScissors::Rock, 'X') => RockPaperScissors::Scissors,
        (RockPaperScissors::Rock, 'Y') => RockPaperScissors::Rock,
        (RockPaperScissors::Rock, 'Z') => RockPaperScissors::Paper,


        (RockPaperScissors::Paper, 'X') => RockPaperScissors::Rock,
        (RockPaperScissors::Paper, 'Y') => RockPaperScissors::Paper,
        (RockPaperScissors::Paper, 'Z') => RockPaperScissors::Scissors,


        (RockPaperScissors::Scissors, 'X') => RockPaperScissors::Paper,
        (RockPaperScissors::Scissors, 'Y') => RockPaperScissors::Scissors,
        (RockPaperScissors::Scissors, 'Z') => RockPaperScissors::Rock,

        _ => { panic!("Invalid character")}
    };

    let point = match player_move {
        RockPaperScissors::Rock => 1,
        RockPaperScissors::Paper => 2,
        RockPaperScissors::Scissors => 3,
    };

    win_point + point
}
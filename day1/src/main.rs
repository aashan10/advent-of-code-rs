use std::fs;
use std::env;

fn main() {
    let elves = read_calories_from_file();

    let (calories, index) = find_max_calories(&elves);
    println!("Max calories: {} for elf {}", calories, index);

    let top_three = find_top_three_calories(&elves);

    let top_three_total: usize = top_three.iter().map(|x| x.0).sum();
    println!("Top three total: {}", top_three_total);
    for (i, calorie) in top_three.iter().enumerate() {
        println!("{}: {} for elf {}", i, calorie.0, calorie.1);
    }

}


fn find_sum_of_calories(elves: &Vec<usize>) -> usize {
    let mut sum = 0;
    for calorie in elves.iter() {
        sum += calorie;
    }
    sum
}

fn find_max_calories(elves: &Vec<Vec<usize>>) -> (usize, usize) {

    let mut max_calories = 0;
    let mut max_elf = 0;

    for (i, elf) in elves.iter().enumerate() {
        let score = find_sum_of_calories(elf);
        if score > max_calories {
            max_calories = score;
            max_elf = i;
        }
    }
    
    (max_calories, max_elf)
}

fn read_calories_from_file() -> Vec<Vec<usize>> {
    let file_path = env::current_dir().unwrap();
    let p = file_path.join("src/calories.txt");
    

    let contents = fs::read_to_string(p)
        .expect("Something went wrong reading the file");

    let mut elves: Vec<Vec<usize>> = vec![];

    let mut current_elf: Vec<usize> = vec![];
    for line in contents.lines() {
        if line.len() > 0 {
            let calorie: usize = line.parse().unwrap();
            current_elf.push(calorie);
        } else {
            elves.push(current_elf);
            current_elf = vec![];
        }
    }
    elves
}

fn find_top_three_calories(elves: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {

    let mut sum_vec = vec![];

    for (i, elf) in elves.iter().enumerate() {
        let score = find_sum_of_calories(elf);
        sum_vec.push((score, i));
    }

    sum_vec.sort_by(|a, b| b.0.cmp(&a.0));



    sum_vec[0..3].to_vec()
}
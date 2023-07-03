use std::{env, fs};

fn main() {
    let compartments = get_compartments_from_file();
    let mut sum = 0;
    for compartment in compartments.iter() {
        let diff_char = find_diff_characters(compartment);
        match diff_char {
            Some(c) => {
                print!("Diff char: {}\n", c);
                sum += get_item_priority(c)
            },
            None => println!("No diff char found"),
        }
    }
    print!("Sum: {}", sum);
}


fn get_item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 96,
        'A'..='Z' => item as u32 - 38,
        _ => panic!("Invalid item"),
    }
}

fn find_diff_characters((a, b): &(String, String)) -> Option<char> {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    for a_char in a_chars.iter() {
        if b_chars.contains(a_char) {
            return Some(*a_char)
        }
    }

    None
}

fn get_compartments_from_file() -> Vec<(String, String)> {
    let mut compartments: Vec<(String, String)> = Vec::new();
    let file_path = env::current_dir().unwrap();
    let p = file_path.join("src/compartments.txt");
    
    
    let contents = fs::read_to_string(p)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let length = line.len();
        let first_compartment = line[0..length/2].to_string();
        let second_compartment = line[length/2..length].to_string();

        compartments.push((first_compartment, second_compartment));
    };

    compartments
    
}
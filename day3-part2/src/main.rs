use std::{env, fs};

fn main() {
    let compartments = get_compartments_from_file();
    let mut sum = 0;

    for compartment in compartments {
        let common = get_common_character(&compartment);
        sum += get_item_priority(common);

    }
    
    println!("Sum: {}", sum);
}

fn get_item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 96,
        'A'..='Z' => item as u32 - 38,
        _ => panic!("Invalid item"),
    }
}

fn get_compartments_from_file() -> Vec<(String, String, String)> {
    let mut compartments: Vec<(String, String, String)> = Vec::new();
    let file_path = env::current_dir().unwrap();
    let p = file_path.join("src/compartments.txt");


    let content = fs::read_to_string(p)
        .expect("Something went wrong reading the file");
    let lines = content.split("\n").collect::<Vec<&str>>();

    let mut i = 0;

    loop {
        if i +2  >= lines.len() {
            break;
        }

        compartments.push((
            lines[i].to_string(),
            lines[i+1].to_string(),
            lines[i+2].to_string()
        ));

        i+=3;        
    }
    


    compartments

}

fn get_common_character((a, b, c): &(String, String, String)) -> char {
    let mut common_between_a_b : Vec<char> = Vec::new();
    let mut common_between_a_b_c : Vec<char> = Vec::new();

    for character in a.chars() {
        if b.contains(character) {
            common_between_a_b.push(character);
        }
    }

    for character in common_between_a_b {
        if c.contains(character) {
            common_between_a_b_c.push(character);
        }
    }

    common_between_a_b_c[0]
}
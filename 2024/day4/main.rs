use std::fs;


fn read_file(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }

    lines
}

fn get_all_combinations(lines: &Vec<String>) -> Vec<String> {
    let mut combinations: Vec<String> = Vec::new();
    const LENGTH_OF_XMAS: usize = 4;

    // Horizontal combinations
    for row in 0..lines.len() {
        for col in 0..=lines[row].len() - LENGTH_OF_XMAS {
            let combination = lines[row][col..col+LENGTH_OF_XMAS].to_string();
            combinations.push(combination);
        }
    }

    // Vertical combinations
    let num_rows = lines.len() - 4;
    for row in 0..=num_rows {
        for col in 0..lines[row].len() {
            let mut combination = String::new();
            for i in 0..LENGTH_OF_XMAS {
                combination.push(lines[row+i].chars().nth(col).unwrap());
            }
            combinations.push(combination);
        }
    }

    // Diagonal combinations
    for row in 0..=lines.len() - LENGTH_OF_XMAS {
        for col in 0..=lines[row].len() - LENGTH_OF_XMAS {
            let mut combination = String::new();
            for i in 0..LENGTH_OF_XMAS {
                combination.push(lines[row+i].chars().nth(col+i).unwrap());
            }
            combinations.push(combination);
        }
    }

    // Reverse diagonal combinations
    for row in 0..=lines.len() - LENGTH_OF_XMAS {
        for col in LENGTH_OF_XMAS-1..lines[row].len() {
            let mut combination = String::new();
            for i in 0..LENGTH_OF_XMAS {
                combination.push(lines[row+i].chars().nth(col-i).expect("No char found"));
            }
            combinations.push(combination);
        }
    }

    combinations
}

fn count_num_xmas_combinations(lines: &Vec<String>) -> u32 {
    let combinations = get_all_combinations(lines);
    let mut count = 0;
    for combination in combinations {
        if combination == "XMAS" || combination == "SAMX" {
            count += 1;
        }
    }
    count
}

#[test]
fn test_get_all_combinations_with_example_text() {
    let lines = read_file("day4/example.txt");
    println!("length {}", lines.len());
    assert_eq!(count_num_xmas_combinations(&lines), 18);
}

fn get_mas_combinations(lines: &Vec<String>) -> u32 {
    let mut count = 0;

    // Move an X pattern over the text
    for row in 0..=lines.len() - 3 {
        for col in 0..=lines[row].len() - 3 {
            // Top left to bottom right
            let mut diagonal = String::new();

            // Top right to bottom left
            let mut reverse_diagonal = String::new();

            for x_index in 0..=2 {
                diagonal.push(lines[row+x_index].chars().nth(col+x_index).unwrap());
                reverse_diagonal.push(lines[row+x_index].chars().nth(col+2-x_index).unwrap());
            }
            if (diagonal == "MAS" || diagonal == "SAM") && (reverse_diagonal == "MAS" || reverse_diagonal == "SAM") {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn test_get_mas_combinations() {
    let lines = read_file("day4/example.txt");
    assert_eq!(get_mas_combinations(&lines), 9);
}

fn main() {
    let lines = read_file("day4/input.txt");
    println!("{:?}", lines);
    let count = count_num_xmas_combinations(&lines);
    println!("{}", count);

    let count = get_mas_combinations(&lines);
    println!("{}", count);
}

use std::{fs, path::Path};

const MAX_ALLOWED_GAP: u32 = 3;

fn read_file(path: &str) -> Vec<Vec<u32>> {
    let file_path = Path::new(path);
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut lines: Vec<Vec<u32>> = Vec::new();

    for line in contents.lines() {
        let parts = line.split(" ");
        let mut line_as_numbers: Vec<u32> = Vec::new();
        for part in parts {
            line_as_numbers.push(part.parse::<u32>().unwrap());
        }
        lines.push(line_as_numbers);
    }

    lines
}

fn is_strictly_monotonic(line: &Vec<u32>) -> bool {
    let mut has_increase = false;
    let mut has_decrease = false;
    for i in 0..line.len() - 1 {
        if line[i] < line[i + 1] {
            has_increase = true;
        } else if line[i] > line[i + 1] {
            has_decrease = true;
        } else if line[i] == line[i + 1] {
            return false;
        }
    }
    has_increase && !has_decrease || !has_increase && has_decrease
}

fn has_adjacent_gap(line: &Vec<u32>, max_allowed_gap: u32) -> bool {
    for i in 0..line.len() - 1 {
        if line[i].abs_diff(line[i + 1]) > max_allowed_gap {
            return true;
        }
    }
    false
}

fn is_safe(line: &Vec<u32>, max_allowed_gap: u32) -> bool {
    is_strictly_monotonic(line) && !has_adjacent_gap(line, max_allowed_gap)
}

fn is_safe_with_dampening(line: &Vec<u32>, max_allowed_gap: u32) -> bool {
    for i in 0..line.len() {
        let left = line[..i].to_vec();
        let right = line[i+1..].to_vec();
        let joined = [&left[..], &right[..]].concat();

        if is_safe(&joined, max_allowed_gap) {
            return true;
        }
    }
    false
}

#[test]
fn test_example() {
    let example_data = read_file("day2/example.txt");

    assert!(is_strictly_monotonic(&example_data[0]));
    assert!(!has_adjacent_gap(&example_data[0], MAX_ALLOWED_GAP));

    assert!(is_strictly_monotonic(&example_data[1]));
    assert!(has_adjacent_gap(&example_data[1], MAX_ALLOWED_GAP));

    assert!(is_strictly_monotonic(&example_data[2]));
    assert!(has_adjacent_gap(&example_data[2], MAX_ALLOWED_GAP));

    assert!(!is_strictly_monotonic(&example_data[3]));
    assert!(!has_adjacent_gap(&example_data[3], MAX_ALLOWED_GAP));

    assert!(!is_strictly_monotonic(&example_data[4]));
    assert!(!has_adjacent_gap(&example_data[4], MAX_ALLOWED_GAP));

    assert!(is_strictly_monotonic(&example_data[5]));
    assert!(!has_adjacent_gap(&example_data[5], MAX_ALLOWED_GAP));
}

#[test]
fn test_example_with_dampening() {
    let example_data = read_file("day2/example.txt");

    assert!(is_safe(&example_data[0], MAX_ALLOWED_GAP));
    assert!(is_safe_with_dampening(&example_data[0], MAX_ALLOWED_GAP));

    assert!(!is_safe(&example_data[1], MAX_ALLOWED_GAP));
    assert!(!is_safe_with_dampening(&example_data[1], MAX_ALLOWED_GAP));

    assert!(!is_safe(&example_data[2], MAX_ALLOWED_GAP));
    assert!(!is_safe_with_dampening(&example_data[2], MAX_ALLOWED_GAP));

    assert!(!is_safe(&example_data[3], MAX_ALLOWED_GAP));
    assert!(is_safe_with_dampening(&example_data[3], MAX_ALLOWED_GAP));

    assert!(!is_safe(&example_data[4], MAX_ALLOWED_GAP));
    assert!(is_safe_with_dampening(&example_data[4], MAX_ALLOWED_GAP));

    assert!(is_safe(&example_data[5], MAX_ALLOWED_GAP));
    assert!(is_safe_with_dampening(&example_data[5], MAX_ALLOWED_GAP));
}

#[test]
fn test_input_examples() {
    let input_data = vec![68, 69, 69, 70, 71, 74, 75, 73];
    assert!(!is_safe(&input_data, MAX_ALLOWED_GAP));
    assert!(!is_safe_with_dampening(&input_data, MAX_ALLOWED_GAP));

    // 91 88 86 85 82 82
    let input_data = vec![91, 88, 86, 85, 82, 82];
    assert!(!is_safe(&input_data, MAX_ALLOWED_GAP));
    assert!(is_safe_with_dampening(&input_data, MAX_ALLOWED_GAP));

    // 33 36 37 34 39
    let input_data = vec![33, 36, 37, 34, 39];
    assert!(!is_safe(&input_data, MAX_ALLOWED_GAP));
    assert!(is_safe_with_dampening(&input_data, MAX_ALLOWED_GAP));

    // 83 83 85 82 84 85 84
    let input_data = vec![83, 83, 85, 82, 84, 85, 84];
    assert!(!is_safe(&input_data, MAX_ALLOWED_GAP));
    assert!(!is_safe_with_dampening(&input_data, MAX_ALLOWED_GAP));

    // Example where we need to remove the last element
    // 83 83 85 82 84 85 84 85
    let input_data = vec![1, 2, 3, 4, 5, 3];
    assert!(!is_safe(&input_data, MAX_ALLOWED_GAP));
    assert!(is_safe_with_dampening(&input_data, MAX_ALLOWED_GAP));
}

fn main() {
    let lines = read_file("day2/input.txt");

    println!("Number of lines: {}", lines.len());

    let mut num_safe = 0;

    for line in &lines {
        num_safe += if is_safe(line, MAX_ALLOWED_GAP) {
            1
        } else {
            0
        };
    }

    println!("Number of safe lines: {num_safe}");

    let mut num_safe_with_dampening = 0;

    for line in &lines {
        num_safe_with_dampening += if is_safe_with_dampening(line, MAX_ALLOWED_GAP) { 1 } else { 0 };
    }

    println!("Number of safe lines with dampening: {num_safe_with_dampening}");
}

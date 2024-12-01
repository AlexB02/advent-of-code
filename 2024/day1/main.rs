use std::fs;
use std::collections::HashMap;
use std::path::Path;

fn get_input_lists() -> (Vec<u32>, Vec<u32>) {
    let file_path = Path::new("day1/input.txt");
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut left_list: Vec<String> = Vec::new();
    let mut right_list: Vec<String> = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split("   ");
        left_list.push(parts.next().unwrap().to_string());
        right_list.push(parts.next().unwrap().to_string());
    }

    assert_eq!(left_list.len(), 1000);
    assert_eq!(right_list.len(), 1000);

    left_list.sort();
    right_list.sort();

    let left_list: Vec<u32> = left_list.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let right_list: Vec<u32> = right_list.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    (left_list, right_list)
}

fn calculate_differences(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    let mut differences: u32 = 0;
    for i in 0..left_list.len() {
        differences += left_list[i].abs_diff(right_list[i]);
    }
    differences
}

fn calculate_similarity(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    let right_list_appearances = num_appearances(right_list);
    let mut similarity: u32 = 0;
    for i in 0..left_list.len() {
        let count = right_list_appearances.get(&left_list[i]).or(Some(&0)).unwrap();
        similarity += count * left_list[i];
    }
    similarity
}

fn num_appearances(list: &Vec<u32>) -> HashMap<u32, u32> {
    list.iter().fold(HashMap::new(), |mut acc, x| {
        let count = acc.entry(*x).or_insert(0);
        *count += 1;
        acc
    })
}

#[test]
fn test_difference() {
    let mut example_left_list: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
    let mut example_right_list: Vec<u32> = vec![4, 3, 5, 3, 9, 3];

    example_left_list.sort();
    example_right_list.sort();

    assert_eq!(example_left_list[0], 1);
    assert_eq!(example_right_list[0], 3);
    assert_eq!(example_left_list[0].abs_diff(example_right_list[0]), 2);

    assert_eq!(calculate_differences(&example_left_list, &example_right_list), 11)
}

#[test]
fn test_similarity() {
    let example_left_list: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
    let example_right_list: Vec<u32> = vec![4, 3, 5, 3, 9, 3];

    assert_eq!(calculate_similarity(&example_left_list, &example_right_list), 31)
}

fn main() {
    let (left_list, right_list) = get_input_lists();

    let differences: u32 = calculate_differences(&left_list, &right_list);
    println!("Difference score: {differences}");

    let similarity: u32 = calculate_similarity(&left_list, &right_list);
    println!("Similarity score: {similarity}");
}

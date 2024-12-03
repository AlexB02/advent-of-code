use regex::Regex;

const MUL_REGEX: &str = r"mul\(([0-9]+),([0-9]+)\)";
const REGEX: &str = r"(mul\(([0-9]+),([0-9]+)\))|(don't\(\))|(do\(\))";

fn find_all_expressions(expression: &str) -> Vec<Operation> {
    let re = Regex::new(REGEX).unwrap();
    re.find_iter(expression).map(|m| {
        let matched = m.as_str();
        if matched.starts_with("mul") {
            let captures = Regex::new(MUL_REGEX).unwrap().captures(matched).unwrap();
            let left = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let right = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            Operation::Mul(left, right)
        } else if matched.starts_with("don't") {
            Operation::DonT
        } else if matched.starts_with("do") {
            Operation::Do
        } else {
            panic!("Failed to match expression {expression}");
        }
    }).collect()
}

#[test]
fn test_matching_examples() {
    let expression = "mul(1,2) + 3 * 4 + mul(5,6)";
    let mul_expressions = find_all_expressions(expression);
    assert_eq!(mul_expressions.len(), 2);
    assert_eq!(mul_expressions[0], Operation::Mul(1, 2));
    assert_eq!(mul_expressions[1], Operation::Mul(5,6));

    let expression = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let mul_expressions = find_all_expressions(expression);
    assert_eq!(mul_expressions.len(), 4);
    assert_eq!(mul_expressions[0], Operation::Mul(2, 4));
    assert_eq!(mul_expressions[1], Operation::Mul(5, 5));
    assert_eq!(mul_expressions[2], Operation::Mul(11, 8));
    assert_eq!(mul_expressions[3], Operation::Mul(8, 5));
}

#[test]
fn test_non_matching_examples() {
    let expressions = vec!("mul(4*", "mul(6,9!", "?(12,34)", "mul ( 2 , 4 )");
    for expression in expressions {
        let mul_expressions = find_all_expressions(expression);
        assert_eq!(mul_expressions.len(), 0);
    }
}

fn parse_and_evaluate_expression(expression: &str) -> i32 {
    let expressions = find_all_expressions(expression);
    // Work out how many of each type
    let mut result = 0;
    let mut enabled = true;
    for expression in expressions {
        match expression {
            Operation::Mul(left, right) => result += if enabled { left * right } else { 0 },
            Operation::Do => enabled = true,
            Operation::DonT => enabled = false,
        }
    }
    result
}

#[test]
fn test_parse_and_evaluate_expression() {
    assert_eq!(parse_and_evaluate_expression("mul(1,2)"), 1 * 2);
    assert_eq!(parse_and_evaluate_expression("mul(1,2) + 3 * 4 + mul(5,6)"), 1 * 2 + 5 * 6);
    assert_eq!(parse_and_evaluate_expression("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5);
}

#[test]
fn test_parse_and_evaluate_expression_with_dos_and_donts() {
    assert_eq!(parse_and_evaluate_expression("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 2 * 4 + 8 * 5);
}

#[test]
fn test_many_dos_and_donts() {
    assert_eq!(parse_and_evaluate_expression("do()do()do()do()don't()mul(1,2)"), 0);
    assert_eq!(parse_and_evaluate_expression("don't()don't()don't()don't()don't()do()mul(1,2)"), 2);
}

fn read_file() -> Vec<String> {
    let file = std::fs::read_to_string("day3/input.txt").unwrap();
    file.lines().map(|x| x.to_string()).collect()
}

#[derive(Debug)]
enum Operation {
    Mul(i32, i32),
    Do,
    DonT,
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operation::Mul(left1, right1), Operation::Mul(left2, right2)) => left1 == left2 && right1 == right2,
            (Operation::Do, Operation::Do) => true,
            (Operation::DonT, Operation::DonT) => true,
            _ => false,
        }
    }
}

fn main() {
    let expressions = read_file();
    let full_expression = expressions.join(" ");
    let total = parse_and_evaluate_expression(&full_expression);
    println!("{}", total);
}

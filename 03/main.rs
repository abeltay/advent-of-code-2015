use std::collections::HashSet;
use std::fs;

fn parse_input(filename: &str) -> String {
    return fs::read_to_string(filename)
        .expect("File not found");
}

fn walk(path: String) -> HashSet<String> {
    let mut x = 0;
    let mut y = 0;
    let mut visited: HashSet<String> = HashSet::new();
    let coord = format!("{x},{y}");
    visited.insert(coord);
    for i in path.chars() {
        match i {
            '^' => {
                y += 1;
            }
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            _ => {
                y -= 1;
            }
        }
        let coord = format!("{x},{y}");
        visited.insert(coord);
    }
    visited
}

fn part1(filename: &str) -> usize {
    let input = parse_input(filename);
    let path = walk(input);
    return path.len();
}

fn part2(filename: &str) -> usize {
    let input = parse_input(filename);
    let mut first = String::new();
    let mut second = String::new();
    for (i, v) in input.chars().enumerate() {
        if i % 2 == 0 {
            first.push(v)
        } else {
            second.push(v)
        }
    }
    let mut santa = walk(first);
    let robot = walk(second);
    santa.extend(robot);
    return santa.len();
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename);
    assert_eq!(answer, 2);
    let answer = part2(filename);
    assert_eq!(answer, 11);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", part1(filename));
    println!("Part 2: {}", part2(filename));
}

use md5;
use std::fs;

fn parse_input(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("File not found");

    return contents.trim().to_string();
}

fn solve(input: String, starts_with: &str) -> u32 {
    let mut postfix = 0;
    loop {
        let mut owned_string: String = input.to_owned();
        owned_string.push_str(&postfix.to_string());
        let digest = md5::compute(owned_string);
        if format!("{:x}", digest).to_string().starts_with(starts_with) {
            return postfix;
        }
        postfix += 1;
    };
}

fn part1(filename: &str) -> u32 {
    let input = parse_input(filename);
    return solve(input, "00000");
}

fn part2(filename: &str) -> u32 {
    let input = parse_input(filename);
    return solve(input, "000000");
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename);
    assert_eq!(answer, 609043);
    let answer = part2(filename);
    assert_eq!(answer, 6742839);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", part1(filename));
    println!("Part 2: {}", part2(filename));
}

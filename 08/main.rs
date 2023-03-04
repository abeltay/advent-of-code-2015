use std::fs;

fn parse_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("File not found");
    let mut lines = Vec::new();
    for line in contents.trim().split('\n') {
        lines.push(line.to_string());
    }
    return lines;
}


fn non_escaped_string(escaped: &String) -> String {
    let mut s = escaped.clone();
    s.pop();
    s.remove(0);
    let mut i = 0;
    while i < s.len() {
        let arr: Vec<char> = s.chars().collect();
        if arr[i] == '\\' {
            if arr[i + 1] != 'x' {
                s.remove(i);
            } else {
                s.remove(i);
                s.remove(i);
                s.remove(i);
            }
        }
        i += 1;
    }
    return s;
}

fn part1(filename: &str) -> usize {
    let input = parse_input(filename);
    let mut diff = 0;
    for line in input.iter() {
        let code = line.len();
        let in_memory = non_escaped_string(line).len();
        diff += code - in_memory;
    }
    return diff;
}

fn encoded_string(code: &String) -> String {
    let mut s = code.clone();
    let mut i = 0;
    while i < s.len() {
        let arr: Vec<char> = s.chars().collect();
        if arr[i] == '\\' || arr[i] == '\"' {
            s.insert(i, '\\');
            i += 1;
        }
        i += 1;
    }
    s.insert(0, '"');
    s.push('"');
    return s;
}

fn part2(filename: &str) -> usize {
    let input = parse_input(filename);
    let mut diff = 0;
    for line in input.iter() {
        let code = line.len();
        let encoded = encoded_string(line).len();
        diff += encoded - code;
    }
    return diff;
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename);
    assert_eq!(answer, 12);
    let answer = part2(filename);
    assert_eq!(answer, 19);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", part1(filename));
    println!("Part 2: {}", part2(filename));
}

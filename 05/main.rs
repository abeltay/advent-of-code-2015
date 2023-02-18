use std::fs;

fn parse_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("File not found");
    let mut lines: Vec<String> = Vec::new();
    for line in contents.trim().split('\n') {
        lines.push(line.to_string());
    }
    return lines.to_vec();
}

fn is_nice(s: String) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let disallowed = ["ab", "cd", "pq", "xy"];
    let mut vowel_count = 0;
    let mut has_repeat = false;
    let mut last = ' ';
    for cur in s.chars() {
        if vowels.contains(&cur) {
            vowel_count += 1
        }
        if last == cur {
            has_repeat = true;
        }
        let sub_string = format!("{}{}", last, cur);
        if disallowed.contains(&sub_string.as_str()) {
            return false;
        }
        last = cur;
    }
    if vowel_count < 3 || !has_repeat {
        return false;
    }
    return true;
}

fn part1(filename: &str) -> u16 {
    let input = parse_input(filename);
    let mut count = 0;
    for s in input {
        if is_nice(s) {
            count = count + 1;
        }
    }
    return count;
}

fn is_nice2(characters: Vec<char>) -> bool {
    let mut seen: Vec<String> = vec![String::new(); characters.len()];
    let mut has_repeat = false;
    for idx in 2..characters.len() {
        if seen.contains(&format!("{}{}", characters[idx - 1], characters[idx])) {
            has_repeat = true;
        }
        seen.push(format!("{}{}", characters[idx - 2], characters[idx - 1]));
    }
    let mut has_surround = false;
    for idx in 2..characters.len() {
        if characters[idx] == characters[idx - 2] {
            has_surround = true;
        }
    }
    return has_repeat && has_surround;
}

fn part2(filename: &str) -> u16 {
    let input = parse_input(filename);
    let mut count = 0;
    for s in input {
        if is_nice2(s.chars().collect()) {
            count = count + 1;
        }
    }
    return count;
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename);
    assert_eq!(answer, 2);
    let filename = "input2_test.txt";
    let answer = part2(filename);
    assert_eq!(answer, 2);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", part1(filename));
    println!("Part 2: {}", part2(filename));
}

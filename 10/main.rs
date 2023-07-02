use std::fs;

fn parse_input(filename: &str) -> String {
    let mut contents = fs::read_to_string(filename).expect("File not found");
    contents.pop();
    return contents;
}

fn append(mut out: String, count: usize, current_char: char) -> String {
    out += &count.to_string();
    out.push(current_char);
    return out;
}

fn look_and_say(input: String) -> String {
    let mut current_char = input.chars().next().expect("cannot get first character");
    let mut count = 0;
    let mut out = String::new();
    for i in input.chars() {
        if i == current_char {
            count += 1;
        } else {
            out = append(out, count, current_char);
            count = 1;
            current_char = i;
        }
    }
    return append(out, count, current_char);
}

fn look_and_say_iterate(filename: &str, iterations: usize) -> usize {
    let mut input = parse_input(filename);
    for _ in 0..iterations {
        input = look_and_say(input);
    }
    return input.len();
}

fn main() {
    let filename = "input_test.txt";
    let answer = look_and_say_iterate(filename, 40);
    assert_eq!(answer, 82350);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", look_and_say_iterate(filename, 40));
    println!("Part 2: {}", look_and_say_iterate(filename, 50));
}

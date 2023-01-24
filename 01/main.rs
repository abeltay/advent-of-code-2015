use std::fs;

fn parse_input(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("File not found");

    return contents;
}

fn part1(input: String) -> i32 {
    let mut answer = 0;
    for c in input.chars() {
        if c == '(' {
            answer += 1;
        } else {
            answer -= 1;
        }
    }
    return answer;
}

fn part2(input: String) -> i32 {
    let mut answer = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            answer += 1;
        } else {
            answer -= 1;
        }
        if answer == -1 {
            return (i+1).try_into().unwrap();
        }
    }
    return 0;
}

fn main() {
    let answer = part1("(())".to_string());
    if answer != 0 {
        println!("Error: received answer is {answer}");
        return;
    }
    let answer = part1("))(((((".to_string());
    if answer != 3 {
        println!("Error: received answer is {answer}");
        return;
    }
    let answer = part2("()())".to_string());
    if answer != 5 {
        println!("Error: received answer is {answer}");
        return;
    }

    println!("Test passed, running input");
    let file = "input.txt";
    let parsed = parse_input(file);
    println!("Part 1: {}", part1(parsed.clone()));
    println!("Part 2: {}", part2(parsed.clone()));
}

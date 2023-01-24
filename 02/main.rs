use std::fs;

fn parse_input(filename: &str) -> Vec<(i32, i32, i32)> {
    let contents = fs::read_to_string(filename)
        .expect("File not found");

    let mut lines: Vec<(i32, i32, i32)> = Vec::new();
    for line in contents.trim().split('\n') {
        let mut split = line.split('x');
        let dimension: (i32, i32, i32) = (
            split.next().expect("must be int").parse::<i32>().unwrap(),
            split.next().expect("must be int").parse::<i32>().unwrap(),
            split.next().expect("must be int").parse::<i32>().unwrap());
        lines.push(dimension)
    }

    return lines.to_vec();
}

fn calculate_area(d: (i32, i32, i32)) -> i32 {
    let side1 = d.0 * d.1;
    let side2 = d.1 * d.2;
    let side3 = d.0 * d.2;
    let min = *vec![side1, side2, side3].iter().min().unwrap();
    return 2 * (side1 + side2 + side3) + min;
}

fn part1(filename: &str) -> i32 {
    let input = parse_input(filename);
    let mut answer = 0;
    for i in input {
        answer += calculate_area(i);
    }
    return answer;
}

fn calculate_ribbon(d: (i32, i32, i32)) -> i32 {
    let mut v = [d.0, d.1, d.2];
    v.sort();
    return (2 * (v[0] + v[1])) + (v[0] * v[1] * v[2]);
}

fn part2(filename: &str) -> i32 {
    let input = parse_input(filename);
    let mut answer = 0;
    for i in input {
        answer += calculate_ribbon(i);
    }
    return answer;
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename);
    assert_eq!(answer, 101);
    let answer = part2(filename);
    assert_eq!(answer, 48);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", part1(filename));
    println!("Part 2: {}", part2(filename));
}

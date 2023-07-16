use std::fs;

fn parse_input(filename: &str) -> Vec<(i64, i64, i64)> {
    let contents = fs::read_to_string(filename).expect("File not found");
    let mut lines: Vec<(i64, i64, i64)> = Vec::new();
    for line in contents.trim().split('\n') {
        let mut split = line.split('x');
        let dimension: (i64, i64, i64) = (
            split.next().expect("must be int").parse::<i64>().unwrap(),
            split.next().expect("must be int").parse::<i64>().unwrap(),
            split.next().expect("must be int").parse::<i64>().unwrap());
        lines.push(dimension)
    }
    lines.to_vec()
}

fn part1(filename: &str) -> i64 {
    let input = parse_input(filename);
    println!("{:?}", input);
    0
}

fn _part2(filename: &str) -> i64 {
    println!("{}", filename);
    0
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename);
    assert_eq!(answer, 0);
    // let answer = part2(filename);
    // assert_eq!(answer, 0);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", part1(filename));
    // println!("Part 2: {}", part2(filename));
}

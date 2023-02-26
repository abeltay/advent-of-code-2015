use std::fs;

enum Instruction { On, Off, Toggle }

struct Input {
    instruction: Instruction,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input(filename: &str) -> Vec<Input> {
    let contents = fs::read_to_string(filename)
        .expect("File not found");

    let mut input: Vec<Input> = Vec::new();
    for line in contents.trim().split('\n') {
        let mut split = line.split(' ');
        let instruction = if split.next().expect("must be string") == "turn" {
            if split.next().expect("must be string") == "on" {
                Instruction::On
            } else {
                Instruction::Off
            }
        } else {
            Instruction::Toggle
        };
        let mut start = split.next().expect("must be string").split(",");
        split.next();
        let mut end = split.next().expect("must be string").split(",");
        input.push(Input {
            instruction,
            start: (
                start.next().expect("must be int").parse::<usize>().unwrap(),
                start.next().expect("must be int").parse::<usize>().unwrap()
            ),
            end: (
                end.next().expect("must be int").parse::<usize>().unwrap(),
                end.next().expect("must be int").parse::<usize>().unwrap()
            ),
        });
    }
    return input;
}

fn switch_lights(map: &mut Vec<[bool; 1000]>, input: &Input) {
    for x in input.start.0..=input.end.0 {
        for y in input.start.1..=input.end.1 {
            match input.instruction {
                Instruction::On => map[x][y] = true,
                Instruction::Off => map[x][y] = false,
                _ => map[x][y] = !map[x][y]
            }
        }
    }
}

fn part1(filename: &str) -> i32 {
    let input = parse_input(filename);
    let mut map: Vec<[bool; 1000]> = Vec::new();
    for _ in 0..1000 {
        let line: [bool; 1000] = [false; 1000];
        map.push(line)
    }
    for i in input.iter() {
        switch_lights(&mut map, i);
    }
    let mut count = 0;
    for line in map.iter() {
        for lighted in line.iter() {
            if *lighted {
                count += 1;
            }
        }
    }
    return count;
}

fn nordic_elvish_lights(map: &mut Vec<[i32; 1000]>, input: &Input) {
    for x in input.start.0..=input.end.0 {
        for y in input.start.1..=input.end.1 {
            match input.instruction {
                Instruction::On => map[x][y] += 1,
                Instruction::Off => {
                    if map[x][y] > 0 {
                        map[x][y] -= 1;
                    }
                }
                _ => map[x][y] += 2,
            }
        }
    }
}

fn part2(filename: &str) -> i32 {
    let input = parse_input(filename);
    let mut map: Vec<[i32; 1000]> = Vec::new();
    for _ in 0..1000 {
        let line: [i32; 1000] = [0; 1000];
        map.push(line)
    }
    for i in input.iter() {
        nordic_elvish_lights(&mut map, i);
    }
    let mut count = 0;
    for line in map.iter() {
        for brightness in line.iter() {
            count += brightness;
        }
    }
    return count;
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename);
    assert_eq!(answer, 998996);
    let filename = "input2_test.txt";
    let answer = part2(filename);
    assert_eq!(answer, 2000001);

    println!("Test passed, running Input");
    let filename = "Input.txt";
    println!("Part 1: {}", part1(filename));
    println!("Part 2: {}", part2(filename));
}

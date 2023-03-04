use std::collections::HashMap;
use std::fs;

fn parse_input(filename: &str) -> HashMap<String, String> {
    let contents = fs::read_to_string(filename)
        .expect("File not found");

    let mut instructions = HashMap::new();
    for line in contents.trim().split('\n') {
        let mut split = line.split(" -> ");
        let operation = split.next().expect("must have value");
        let value = split.next().expect("must have value");
        instructions.insert(value.to_string(), operation.to_string());
    }

    return instructions;
}

fn value(input: &HashMap<String, String>, cache: &mut HashMap<String, u16>, wire: &str) -> u16 {
    let num = wire.parse::<u16>();
    match num {
        Ok(ok) => {
            cache.insert(wire.to_string(), ok);
            return ok;
        }
        Err(_) => {}
    };
    if cache.contains_key(&wire.to_string()) {
        return cache.get(&wire.to_string()).expect("must have value").to_owned();
    }
    let instruction = input.get(&wire.to_string()).expect("must have value");
    let split: Vec<&str> = instruction.split(" ").collect();
    if split.len() == 1 {
        let ans = value(input, cache, split[0]);
        cache.insert(wire.to_string(), ans);
        return ans;
    }
    if split[0] == "NOT" {
        let s = value(input, cache, split[1]);
        let v = !s;
        cache.insert(wire.to_string(), v);
        return v;
    }
    let first_val = value(input, cache, split[0]);
    let num = split[2].parse::<u16>();
    match num {
        Ok(ok) => {
            let ans = if split[1] == "LSHIFT" {
                first_val << ok
            } else {
                first_val >> ok
            };
            cache.insert(wire.to_string(), ans);
            return ans;
        }
        Err(_) => {}
    };
    let third_val = value(input, cache, split[2]);
    let ans = if split[1] == "AND" {
        first_val & third_val
    } else {
        first_val | third_val
    };
    cache.insert(wire.to_string(), ans);
    return ans;
}

fn part1(filename: &str, wire: &str) -> u16 {
    let input = parse_input(filename);
    let mut cache: HashMap<String, u16> = HashMap::new();
    return value(&input, &mut cache, wire);
}

fn part2(filename: &str, wire: &str) -> u16 {
    let mut input = parse_input(filename);
    let mut cache: HashMap<String, u16> = HashMap::new();
    let first_run = value(&input, &mut cache, wire);
    input.insert("b".to_string(), first_run.to_string());
    let mut cache: HashMap<String, u16> = HashMap::new();
    return value(&input, &mut cache, wire);
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename, "h");
    assert_eq!(answer, 65412);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", part1(filename, "a"));
    println!("Part 2: {}", part2(filename, "a"));
}

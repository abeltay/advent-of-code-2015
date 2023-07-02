use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("File not found");
    let mut lines: Vec<(String, String, u32)> = Vec::new();
    for line in contents.trim().split('\n') {
        let mut split = line.split(" = ");
        let first = split.next().expect("must have value");
        let mut loc = first.split(" to ");
        let route: (String, String, u32) = (
            loc.next().expect("must have first location").to_string(),
            loc.next().expect("must have second location").to_string(),
            split.next().expect("must be int").parse::<u32>().unwrap());
        lines.push(route)
    }
    let mut new_set = HashSet::new();
    for i in lines.iter() {
        new_set.insert(i.clone().0);
        new_set.insert(i.clone().1);
    }
    let mut slice2d: Vec<Vec<u32>> = vec![vec![0; new_set.len()]; new_set.len()];
    let mut cities: Vec<String> = new_set.into_iter().collect();
    cities.sort();
    for i in lines.iter() {
        let line = i.clone();
        let first_pos = cities.binary_search(&line.0).expect("should exist");
        let second_pos = cities.binary_search(&line.1).expect("should exist");
        slice2d[first_pos][second_pos] = line.2;
        slice2d[second_pos][first_pos] = line.2;
    }
    return slice2d;
}

fn explore(map: &Vec<Vec<u32>>, visited: HashSet<usize>, loc: usize, current_lowest_cost: u32, cost: u32) -> u32 {
    if visited.len() >= map.len() {
        return min(current_lowest_cost, cost);
    }
    let mut lowest_cost = current_lowest_cost;
    for i in 0..map.len() {
        if visited.contains(&i) {
            continue;
        }
        let next_cost = cost + map[loc][i];
        if next_cost >= current_lowest_cost {
            continue;
        }
        let mut next = visited.clone();
        next.insert(i);
        let new_cost = explore(map, next, i, current_lowest_cost, next_cost);
        lowest_cost = min(lowest_cost, new_cost);
    }
    return lowest_cost;
}

fn part1(filename: &str) -> u32 {
    let map = parse_input(filename);
    let mut cost = u32::MAX;
    for i in 0..map.len() {
        let mut set = HashSet::new();
        set.insert(i);
        let new_cost = explore(&map, set, i, cost, 0);
        cost = min(cost, new_cost);
    }
    return cost;
}

fn explore_longest(map: &Vec<Vec<u32>>, visited: HashSet<usize>, loc: usize, current_highest_cost: u32, cost: u32) -> u32 {
    if visited.len() >= map.len() {
        return max(current_highest_cost, cost);
    }
    let mut highest_cost = current_highest_cost;
    for i in 0..map.len() {
        if visited.contains(&i) {
            continue;
        }
        let next_cost = cost + map[loc][i];
        let mut next = visited.clone();
        next.insert(i);
        let new_cost = explore_longest(map, next, i, current_highest_cost, next_cost);
        highest_cost = max(highest_cost, new_cost);
    }
    return highest_cost;
}

fn part2(filename: &str) -> u32 {
    let map = parse_input(filename);
    let mut cost = 0;
    for i in 0..map.len() {
        let mut set = HashSet::new();
        set.insert(i);
        let new_cost = explore_longest(&map, set, i, cost, 0);
        cost = max(cost, new_cost);
    }
    return cost;
}

fn main() {
    let filename = "input_test.txt";
    let answer = part1(filename);
    assert_eq!(answer, 605);
    let answer = part2(filename);
    assert_eq!(answer, 982);

    println!("Test passed, running input");
    let filename = "input.txt";
    println!("Part 1: {}", part1(filename));
    println!("Part 2: {}", part2(filename));
}

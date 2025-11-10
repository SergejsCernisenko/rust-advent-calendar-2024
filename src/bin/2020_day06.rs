use std::{collections::{HashMap, HashSet}, fs};

fn part1(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let groups: Vec<&str> = contents.split("\n\n").collect();
    let mut groups_unique_answers_sum = 0;
    
    for group in groups.iter() {
        let mut unique_chars = HashSet::new();
        
        for line in group.lines() {
            for ch in line.chars() {
                unique_chars.insert(ch);
            }
        }

        groups_unique_answers_sum += unique_chars.len();
    }

    println!("Sum of unique answers in groups: {}", groups_unique_answers_sum);
}

fn part2(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let groups: Vec<&str> = contents.split("\n\n").collect();
    let mut groups_common_answers_sum = 0;
    
    for group in groups.iter() {
        let mut unique_chars = HashMap::new();
        let lines_count = group.lines().count();
        
        for line in group.lines() {
            for ch in line.chars() {
                unique_chars
                    .entry(ch)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        unique_chars.retain(|_, &mut count| count == lines_count);
        groups_common_answers_sum += unique_chars.len();
    }

    println!("Sum of common answers in groups: {}", groups_common_answers_sum);
}

fn main() {
    println!("Day 6");
    let filename = "src/bin/data/input_2020_day06.txt";
    part1(filename);
    part2(filename);
}
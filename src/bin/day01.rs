use std::{fs, ops::Mul};

fn parse_input(filename: &str) -> (Vec<u32>, Vec<u32>) {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");
    
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    
    for line in contents.lines() {
        let parts: Vec<&str> = line.split("   ").collect(); // 3 spaces
        if parts.len() == 2 {
            col1.push(parts[0].trim().parse().expect("Invalid number"));
            col2.push(parts[1].trim().parse().expect("Invalid number"));
        }
    }
    
    (col1, col2)
}

fn main() {
    println!("Day 1 - Parts 1 and 2");

    let (col1, col2) = parse_input("src/bin/data/input_1.txt");
    part_one(&col1, &col2);
    part_two(&col1, &col2);
}

fn part_one(col1: &[u32], col2: &[u32]) {
    let mut col1 = col1.to_vec();
    let mut col2 = col2.to_vec();
    col1.sort_unstable();
    col2.sort_unstable();

    let mut sum = 0;
    for (i, num) in col1.iter().enumerate() {
        sum += num.abs_diff(col2[i]);
    }

    println!("Result: {sum}");
}

fn part_two(col1: &[u32], col2: &[u32]) {
    let mut sum: u32 = 0;
    for num in col1 {
        let number_appearance_times = u32::try_from(col2.iter().filter(|&&x| x == *num).count()).unwrap_or(0);
        sum += number_appearance_times.mul(num);
    }

    println!("Similarity score: {sum}");
}
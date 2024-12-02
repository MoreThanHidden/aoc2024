use std::fs::File;
use std::io::Read;

pub fn day1_part1() {
    //Take input from a text File
    let mut file = File::open("day1.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split("\n").collect();
    

    // Read the two columns separately into a vector
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    for i in 0..input.len() {
        let temp: Vec<&str> = input[i].split_whitespace().collect();
        col1.push(temp[0].parse().unwrap());
        col2.push(temp[1].parse().unwrap());
    }
    
    // Sort the two columns and add the difference
    col1.sort();
    col2.sort();
    let mut sum = 0;
    for i in 0..input.len() {
        sum += col2[i].abs_diff(col1[i]);
    }
    
    println!("Day 1 Part 1: {}", sum);
    
}

pub fn day1_part2() {
    //Take input from a text File
    let mut file = File::open("day1.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split("\n").collect();


    // Read the two columns separately into a vector
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    for i in 0..input.len() {
        let temp: Vec<&str> = input[i].split_whitespace().collect();
        col1.push(temp[0].parse().unwrap());
        col2.push(temp[1].parse().unwrap());
    }

    // Add the number times the number of times the number appears in the right column
    let mut sum = 0;
    for i in 0..input.len() {
        sum += col1[i] * col2.iter().filter(|&x| *x == col1[i]).count() as i32;
    }
    
    println!("Day 1 Part 2: {}", sum);
    
}
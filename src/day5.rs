use std::fs::File;
use std::io::Read;

pub fn day5_part1() {
    //Take input from a text File
    let mut file = File::open("day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut sum = 0;

    let input: Vec<&str> = input.split("\r\n\r\n").collect();

    // Rules into a vector of tuples
    let rules: Vec<(i32, i32)> = input[0].split("\r\n").map(|x| {
        let temp: Vec<&str> = x.split("|").collect();
        (temp[0].parse().unwrap(), temp[1].parse().unwrap())
    }).collect();

    // Updates into a vector of vectors
    let updates: Vec<Vec<i32>> = input[1].split("\r\n").map(|x| {
        x.split(",").map(|y| y.parse().unwrap()).collect()
    }).collect();

    //Loop through the updates
    for update in updates {
        let mut valid = true;
        for i in 0..update.len() {
            for rule in &rules {
                // Check that the second number in the rule is not before the first number
                if update[i] == rule.0 && update[0..i].iter().find(|&&x| x == rule.1).is_some() {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            // Add the middle number to the sum
            sum += update[update.len() / 2];
        }
    }

    println!("Day 5 Part 1: {}", sum);
}

pub fn day5_part2() {
    //Take input from a text File
    let mut file = File::open("day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut sum = 0;

    let input: Vec<&str> = input.split("\r\n\r\n").collect();

    // Rules into a vector of tuples
    let rules: Vec<(i32, i32)> = input[0].split("\r\n").map(|x| {
        let temp: Vec<&str> = x.split("|").collect();
        (temp[0].parse().unwrap(), temp[1].parse().unwrap())
    }).collect();

    // Updates into a vector of vectors
    let updates: Vec<Vec<i32>> = input[1].split("\r\n").map(|x| {
        x.split(",").map(|y| y.parse().unwrap()).collect()
    }).collect();

    //Loop through the updates
    for mut update in updates {
        let mut valid = true;
        let mut changed = false;
        // Loop until no more swaps need to be made
        while valid {
            valid = false;
            for i in 0..update.len() {
                for rule in &rules {
                    // Check that the second number in the rule is not before the first number
                    if update[i] == rule.0 {
                        for j in 0..i {
                            if update[j] == rule.1 {
                                valid = true;
                                changed = true;
                                // Swap the numbers
                                update.swap(i, j);
                                break;
                            }
                        }
                    }
                }
            }
        }
        if changed {
            // Add the middle number to the sum
            sum += update[update.len() / 2];
        }
    }

    println!("Day 5 Part 2: {}", sum);
}
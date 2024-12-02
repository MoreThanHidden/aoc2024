use std::fs::File;
use std::io::Read;

pub fn day2_part1() {
    //Take input from a text File
    let mut file = File::open("day2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split("\n").collect();
    
    // Each Row can either be increasing or decreasing not the same or changing order
    let mut sum = 0;
    for i in 0..input.len() {
        if test_string(input[i].to_string()) {
            sum += 1;
        }
    }
    
    println!("Day 2 Part 1: {}", sum);
    
}

pub fn test_string(input : String) -> bool{
    let report: Vec<&str> = input.split_whitespace().collect();
    let mut is_ascending = true;
    let mut is_descending = true;
    let mut is_diff3 = false;

    for j in 1..report.len() {
        let current = report[j].parse::<i32>().unwrap();
        let last = report[j - 1].parse::<i32>().unwrap();

        if current.abs_diff(last) > 3 {
            is_diff3 = true;
        }

        if current > last {
            is_descending = false;
        } else if current < last {
            is_ascending = false;
        } else {
            is_ascending = false;
            is_descending = false;
            break;
        }
    }

    (is_ascending || is_descending) && !is_diff3
}


pub fn day2_part2() {
    //Take input from a text File
    let mut file = File::open("day2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split("\n").collect();
    
    
    let mut sum = 0;
    //same as part 1 but now any 1 number can be removed to make it safe
    for i in 0..input.len() {
        let report: Vec<&str> = input[i].split_whitespace().collect();

        for j in 0..report.len() {
            let mut temp = report.clone();
            temp.remove(j);
            if test_string(temp.join(" ").to_string()) {
                sum += 1;
                break;
            }
        }

    }
    
    println!("Day 2 Part 2: {}", sum);
    
}
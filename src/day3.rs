use std::fs::File;
use std::io::Read;

pub fn day3_part1() {
    //Take input from a text File
    let mut file = File::open("day3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    
    let sum = test_muls(input);
    
    println!("Day 3 Part 1: {}", sum);
}

fn test_muls (input: String) -> i32 {
    let mut sum = 0;
    let input: Vec<&str> = input.split("mul(").collect();
    for i in 0..input.len() {
        if input[i].contains(")") {
            let temp2: Vec<&str> = input[i].split(")").collect();
            if input[i].contains(",") {
                let temp3: Vec<&str> = temp2[0].split(",").collect();
                //check if the length is 2
                if(temp3.len() == 2) {
                    //check if the two values are numbers
                    let x = temp3[0].parse::<i32>();
                    let y = temp3[1].parse::<i32>();
                    if x.is_ok() && y.is_ok() {
                        sum += x.unwrap() * y.unwrap();
                    }
                }
            }
        }
    }
    sum
}

pub fn day3_part2() {
    //Take input from a text File
    let mut file = File::open("day3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    
    let mut sum = 0;
    // The do() instruction enables future mul instructions.
    // The don't() instruction disables future mul instructions.
    let dos: Vec<&str> = input.split("do()").collect();
    for i in 0..dos.len() {
        let donts: Vec<&str> = dos[i].split("don't()").collect();
        sum += test_muls(donts[0].to_string());
    }
    
    println!("Day 3 Part 2: {}", sum);
}

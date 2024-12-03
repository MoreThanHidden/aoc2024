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
    let muls = input.split("mul(").skip(1);
    for mul in muls {
        if let Some((args, _)) = mul.split_once(")") {
            if let Some((x, y)) = args.split_once(",") {
                if let (Ok(x), Ok(y)) = (x.trim().parse::<i32>(), y.trim().parse::<i32>()) {
                    sum += x * y;
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

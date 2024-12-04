use std::fs::File;
use std::io::Read;

pub fn day4_part1() {
    //Take input from a text File
    let mut file = File::open("day4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut sum = 0;

    let stdinput: Vec<&str> = input.split("\r\n").collect();

    //Standard Left to Right input
    for i in 0..stdinput.len() {
        sum += count_xmas(&stdinput[i].to_string());
    }

    //Downward
    for i in 0..stdinput[0].len(){
        let mut temp = String::new();
        for row in &stdinput {
            temp.push(row.chars().nth(i).unwrap());
        }
        sum += count_xmas(&temp);
    }

    //Diagonal
    for i in 0..stdinput.len() {
        let mut temp = String::new();
        let mut temp2 = String::new();
        let mut temp3 = String::new();
        let mut temp4 = String::new();

        for j in 0..stdinput[0].len() {
            if i + j < stdinput.len() {
                //Diagonal Bottom Left to Right
                temp.push(stdinput[i + j].chars().nth(j).unwrap());
                //Diagonal Bottom Right to Left
                temp2.push(stdinput[i + j].chars().nth(stdinput[0].len() - j - 1).unwrap());
            }
            if i < (stdinput.len() - 1) && !i.overflowing_sub(j).1 {
                //Diagonal Top Left to Right
                temp3.push(stdinput[i - j].chars().nth(j).unwrap());
                //Diagonal Top Right to Left
                temp4.push(stdinput[i - j].chars().nth(stdinput[0].len() - j - 1).unwrap());
            }
        }

        for temp in [&temp, &temp2, &temp3, &temp4] {
            if temp.len() > 3 {
                sum += count_xmas(temp);
            }
        }
    }

    println!("Day 4 Part 1: {}", sum);

}

fn count_xmas(input: &String) -> i32 {
    let mut sum = 0;
    let chars: Vec<char> = input.chars().collect();
    for i in 0..input.len()-3 {
        if chars[i] == 'X' && chars[i+1] == 'M' && chars[i+2] == 'A' && chars[i+3] == 'S' {
            sum += 1;
        } else if chars[i] == 'S' && chars[i+1] == 'A' && chars[i+2] == 'M' && chars[i+3] == 'X' {
            sum += 1;
        }
    }
    sum
}

pub fn day4_part2() {
    //Take input from a text File
    let mut file = File::open("day4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut sum = 0;
    
    let stdinput: Vec<&str> = input.split("\r\n").collect();
    
    // Find the As and look around for S and M Diagonally
    for i in 1..stdinput.len()-1 {
        let chars_up: Vec<char> = stdinput[i-1].chars().collect();
        let chars: Vec<char> = stdinput[i].chars().collect();
        let chars_down: Vec<char> = stdinput[i+1].chars().collect();
        for j in 1..chars.len()-1 {
            if chars[j] == 'A' {
                let mut temp = 0;
                if chars_up[j-1] == 'S' && chars_down[j+1] == 'M' {
                    temp += 1;
                }
                if chars_up[j-1] == 'M' && chars_down[j+1] == 'S' {
                    temp += 1;
                }
                if chars_up[j+1] == 'S' && chars_down[j-1] == 'M' {
                    temp += 1;
                }
                if chars_up[j+1] == 'M' && chars_down[j-1] == 'S' {
                    temp += 1;
                }
                if temp > 1 {
                    sum += 1;
                }
            }
        }
    }
    
    println!("Day 4 Part 2: {}", sum);

}
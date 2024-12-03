use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn main() {
    let file = File::open("./src/day_03/day_03.txt").unwrap();
    // let file = File::open("./src/day_03/example.txt").unwrap();
    let reader = BufReader::new(file);
    let mut input = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        input.push_str(&line);
    }

    println!("{input}");

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let result: Vec<String> = re
        .find_iter(&input)
        .map(|mat| mat.as_str().to_string())
        .collect();
    println!("{result:?}");

    let mut sum = 0;
    let mut enable = true;
    for expr in &result {
        println!("{expr}");
        match expr.as_str() {
            "do()" => {
                enable = true;
                continue;
            },
            "don't()" => {
                enable = false;
                continue;
            }
            _ => {}
        }
        if enable {
            let expr = &expr[4..expr.len() - 1];
            let nums: Vec<&str> = expr.split(',').collect();
            match (nums[0].parse::<usize>(), nums[1].parse::<usize>()) {
                (Ok(num1), Ok(num2)) => sum += num1 * num2,
                _ => {}
            }
        }
    }
    println!("result: {sum}");
}
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let file = File::open("./src/day_01/day_01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut list1: BinaryHeap<usize> = BinaryHeap::new();
    let mut list2: BinaryHeap<usize> = BinaryHeap::new();

    let mut frequency1: HashMap<usize, usize> = HashMap::new();
    let mut frequency2: HashMap<usize, usize> = HashMap::new();

    for line in reader.lines() {
        // println!("{line:?}");
        let nums = line.unwrap().split_whitespace().map(|word| word.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        // println!("{:?}, {:?}", nums[0], nums[1]);
        list1.push(nums[0]);
        list2.push(nums[1]);

        let freq = frequency1.entry(nums[0]).or_insert(0);
        *freq += 1;
        let freq = frequency2.entry(nums[1]).or_insert(0);
        *freq += 1;
    }

    let mut distance = 0;
    while !list1.is_empty() {
        let el1 = list1.pop().unwrap();
        let el2 = list2.pop().unwrap();
        distance += el1.abs_diff(el2);
    }
    println!("Total distance: {distance}");

    let mut similarity = 0;
    for (num1, _freq1) in &frequency1 {
        let freq2 = frequency2.get(num1).unwrap_or(&0);
        similarity += num1 * freq2;
    }
    println!("Total similarity: {similarity}");
}
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let file = File::open("./src/day_02/day_02.txt").unwrap();
    // let file = File::open("./src/day_02/example.txt").unwrap();
    // let file = File::open("./src/day_02/test.txt").unwrap();
    let reader = BufReader::new(file);

    let mut reports = Vec::new();
    for line in reader.lines() {
        let report = line.unwrap().split_whitespace().map(|word| word.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        reports.push(report);
    }

    // part 1
    let mut valid_reports = 0;
    'outer: for report in &mut reports {
        let is_increasing = is_safe(report);
        report.reverse();
        let is_decreasing = is_safe(report);
        if is_increasing || is_decreasing {
            valid_reports += 1;
        }
        /*
        println!("{report:?}");
        let mut is_increasing = true;
        let mut is_decreasing = true;
        for (first, second) in report.iter().zip(report.iter().skip(1)) {
            let diff = first - second;
            if !(1..=3).contains(&diff.abs()) {
                continue 'outer;
            }
            if first < second {
                is_decreasing = false;
            }
            if first > second {
                is_increasing = false;
            }
        }
        if is_increasing || is_decreasing {
            valid_reports += 1;
        }
         */
    }
    // println!("Valid reports: {valid_reports}");

    // part 2
    let mut valid_reports = 0;
    for report in &mut reports {
        let is_increasing = is_safe_with_removing(report) || is_safe(&report[1..]) || is_safe(&report[..report.len() - 1]);
        report.reverse();
        let is_decreasing = is_safe_with_removing(report) || is_safe(&report[1..]) || is_safe(&report[..report.len() - 1]);
        if is_increasing || is_decreasing {
            println!("{report:?} safe");
            valid_reports += 1;
        } else {
            println!("{report:?} NOT safe");
        }
        /*
        let mut report = line.unwrap().split_whitespace().map(|word| word.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut is_increasing = is_safe_with_removing(&report);
        if report.len() > 0 {
            let mut report_without_first = report.iter().skip(1).copied().collect::<Vec<i32>>();
            let is_safe_without_first = is_safe(&report_without_first);
            report_without_first.reverse();
            let is_safe_without_first = is_safe_without_first || is_safe(&report_without_first);
            println!("{report:?} is safe without first: {is_safe_without_first}");
            is_increasing = is_increasing || is_safe_without_first;
        }
        report.reverse();
        let mut is_decreasing = is_safe_with_removing(&report);
        if report.len() > 0 {
            let mut report_without_first = report.iter().skip(1).copied().collect::<Vec<i32>>();
            let is_safe_without_first = is_safe(&report_without_first);
            report_without_first.reverse();
            let is_safe_without_first = is_safe_without_first || is_safe(&report_without_first);
            println!("{report:?} is safe without first: {is_safe_without_first}");
            is_decreasing = is_decreasing || is_safe_without_first;
        }
        if is_increasing || is_decreasing {
            // println!("{report:?} is safe");
            valid_reports += 1;
        } else {
            println!("{report:?} is NOT safe");
        }
         */
    }
    println!("Valid reports: {valid_reports}");
}

fn is_safe(report: &[i32]) -> bool {
        let mut is_increasing = true;
        for (first, second) in report.iter().zip(report.iter().skip(1)) {
            let diff = first - second;
            if !(1..=3).contains(&diff.abs()) {
                return false;
            }
            if first > second {
                is_increasing = false;
            }
        }
        is_increasing
}


fn is_safe_with_removing(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut temp_report = report.to_vec();
        temp_report.remove(i);

        if is_safe(&temp_report) {
            return true;
        }
    }
    false
}

/*
fn is_safe_with_removing(report: &[i32]) -> bool {
    let mut prev = *report.first().unwrap();
    let mut can_be_excluded = true;
    for value in report.iter().skip(1) {
        let diff = prev.abs_diff(*value);
        if !(1..=3).contains(&diff) {
            if can_be_excluded {
                can_be_excluded = false;
                continue;
            } else {
                return false;
            }
        }

        if prev > *value {
            if can_be_excluded {
                can_be_excluded = false;
                continue;
            } else {
                return false;
            }
        }
        prev = *value;
    }
    true
}
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let file = File::open("./src/day_04/day_04.txt").unwrap();
    // let file = File::open("./src/day_04/test.txt").unwrap();
    // let file = File::open("./src/day_04/example.txt").unwrap();
    let reader = BufReader::new(file);

    let mut table: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let chars = line
            .chars()
            .map(|c| c)
            .collect::<Vec<char>>();
        table.push(chars);
    }

    // part 1
    let mut x_positions: Vec<(i32, i32)> = Vec::new();
    for (i, line) in table.iter().enumerate() {
        for (j, value) in line.iter().enumerate() {
            if *value == 'X' {
                x_positions.push((i as i32, j as i32));
            }
        }
    }

    let mut count = 0;
    check(&table, &x_positions, &mut count);
    println!("count: {count}");

    // part 2
    let mut a_positions: Vec<(i32, i32)> = Vec::new();
    for (i, line) in table.iter().enumerate() {
        for (j, value) in line.iter().enumerate() {
            if *value == 'A' {
                a_positions.push((i as i32, j as i32));
            }
        }
    }

    let mut count = 0;
    check_part2(&table, &a_positions, &mut count);
    println!("count: {count}");
}

#[derive(Debug)]
enum Direction {
    UP(i32, i32),
    UPRIGHT(i32, i32),
    RIGHT(i32, i32),
    DOWNRIGHT(i32, i32),
    DOWN(i32, i32),
    DOWNLEFT(i32, i32),
    LEFT(i32, i32),
    UPLEFT(i32, i32),
}

fn check_part2(table: &Vec<Vec<char>>, a_positions: &Vec<(i32, i32)>, count: &mut usize) {
    fn check_diag(table: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
        let upleft = (position.0 - 1, position.1 - 1);
        let downright = (position.0 + 1, position.1 + 1);
        if upleft.0 < 0 || upleft.1 < 0 {
            return false;
        }
        if downright.0 > table.len() as i32 - 1 || downright.1 > table[0].len() as i32 - 1 {
            return false;
        }
        let upleft = table[upleft.0 as usize][upleft.1 as usize];
        let downright = table[downright.0 as usize][downright.1 as usize];
        (upleft == 'M' && downright == 'S') || (upleft == 'S' && downright == 'M')
    }

    fn check_counter_diag(table: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
        let upright = (position.0 - 1, position.1 + 1);
        let downleft = (position.0 + 1, position.1 - 1);
        if upright.0 < 0 || upright.1 > table[0].len() as i32 - 1 {
            return false;
        }
        if downleft.0 > table.len() as i32 - 1 || downleft.1 < 0 {
            return false;
        }
        let upleft = table[upright.0 as usize][upright.1 as usize];
        let downright = table[downleft.0 as usize][downleft.1 as usize];
        (upleft == 'M' && downright == 'S') || (upleft == 'S' && downright == 'M')
    }

    for (row, col) in a_positions {
        let diag = check_diag(table, (*row, *col));
        let counter_diag = check_counter_diag(table, (*row, *col));
        if diag && counter_diag {
            *count += 1;
        }
    }
}

fn check(table: &Vec<Vec<char>>, x_positions: &Vec<(i32, i32)>, count: &mut usize) {
    let drow: [i32; 8] = [-1, -1, -1,  0, 0,  1, 1, 1];
    let dcol: [i32; 8] = [-1,  0,  1, -1, 1, -1, 0, 1];
    for (row, col) in x_positions {
        for (dr, dc) in drow.iter().zip(dcol.iter()) {
            let direction = match (dr, dc) {
                (-1, -1) => Direction::UPLEFT(*dr, *dc),
                (-1, 0) => Direction::UP(*dr, *dc),
                (-1, 1) => Direction::UPRIGHT(*dr, *dc),
                (0, -1) => Direction::LEFT(*dr, *dc),
                (0, 1) => Direction::RIGHT(*dr, *dc),
                (1, -1) => Direction::DOWNLEFT(*dr, *dc),
                (1, 0) => Direction::DOWN(*dr, *dc),
                (1, 1) => Direction::DOWNRIGHT(*dr, *dc),
                (_, _) => panic!("Invalid direction"),
            };
            check_aux(table, (row + *dr, col + *dc), direction, 3, count);
        }
    }
}

fn check_aux(table: &Vec<Vec<char>>, current_pos: (i32, i32), direction: Direction, chars_left: usize, count: &mut usize) {
    if chars_left == 0 {
        *count += 1;
        return;
    }
    let (current_char, row_offset, col_offset) = match direction {
        Direction::UP(row_offset, col_offset) => {
            let row_index = current_pos.0;
            if row_index < 0 {
                return;
            }
            let row = table.get(row_index as usize).unwrap();
            let value = row.get(current_pos.1 as usize).unwrap();
            (value, row_offset, col_offset)
        }
        Direction::UPRIGHT(row_offset, col_offset) => {
            let row_index = current_pos.0;
            if row_index < 0 {
                return;
            }
            let col_index = current_pos.1;
            if col_index > table[0].len() as i32 - 1 {
                return;
            }
            let row = table.get(row_index as usize).unwrap();
            let value = row.get(col_index as usize).unwrap();
            (value, row_offset, col_offset)
        }
        Direction::RIGHT(row_offset, col_offset) => {
            let col_index = current_pos.1;
            if col_index > table[0].len() as i32 - 1 {
                return;
            }
            let row = table.get(current_pos.0 as usize).unwrap();
            let value = row.get(col_index as usize).unwrap();
            (value, row_offset, col_offset)
        }
        Direction::DOWNRIGHT(row_offset, col_offset) => {
            let row_index = current_pos.0;
            if row_index > table.len() as i32 - 1 {
                return;
            }
            let col_index = current_pos.1;
            if col_index > table[0].len() as i32 - 1 {
                return;
            }
            let row = table.get(row_index as usize).unwrap();
            let value = row.get(col_index as usize).unwrap();
            (value, row_offset, col_offset)
        }
        Direction::DOWN(row_offset, col_offset) => {
            let row_index = current_pos.0;
            if row_index > table.len() as i32 - 1 {
                return;
            }
            let row = table.get(row_index as usize).unwrap();
            let value = row.get(current_pos.1 as usize).unwrap();
            (value, row_offset, col_offset)
        }
        Direction::DOWNLEFT(row_offset, col_offset) => {
            let row_index = current_pos.0;
            if row_index > table.len() as i32 - 1 {
                return;
            }
            let col_index = current_pos.1;
            if col_index < 0 {
                return;
            }
            let row = table.get(row_index as usize).unwrap();
            let value = row.get(col_index as usize).unwrap();
            (value, row_offset, col_offset)
        }
        Direction::LEFT(row_offset, col_offset) => {
            let col_index = current_pos.1;
            if col_index < 0 {
                return;
            }
            let row = table.get(current_pos.0 as usize).unwrap();
            let value = row.get(col_index as usize).unwrap();
            (value, row_offset, col_offset)
        }
        Direction::UPLEFT(row_offset, col_offset) => {
            let row_index = current_pos.0;
            if row_index < 0 {
                return;
            }
            let col_index = current_pos.1;
            if col_index < 0 {
                return;
            }
            let row = table.get(row_index as usize).unwrap();
            let value = row.get(col_index as usize).unwrap();
            (value, row_offset, col_offset)
        }
    };
    match chars_left {
        3 => {
            if *current_char == 'M' {
                let next_pos = (row_offset + current_pos.0, col_offset + current_pos.1);
                check_aux(table, next_pos, direction, chars_left - 1, count);
            }
        },
        2 => {
            if *current_char == 'A' {
                let next_pos = (row_offset + current_pos.0, col_offset + current_pos.1);
                check_aux(table, next_pos, direction, chars_left - 1, count);
            }
        },
        1 => {
            if *current_char == 'S' {
                let next_pos = (row_offset + current_pos.0, col_offset + current_pos.1);
                check_aux(table, next_pos, direction, chars_left - 1, count);
            }
        },
        _ => {
            return;
        }
    }
}
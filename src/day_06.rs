use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    UP(i32, i32),
    RIGHT(i32, i32),
    DOWN(i32, i32),
    LEFT(i32, i32)
}

impl Direction {
    fn get_next(&self) -> Direction {
        match self {
            Direction::UP(_, _) => { Direction::RIGHT(0, 1) }
            Direction::RIGHT(_, _) => { Direction::DOWN(1, 0) }
            Direction::DOWN(_, _) => { Direction::LEFT(0, -1) }
            Direction::LEFT(_, _) => { Direction::UP(-1, 0) }
        }
    }

    fn get_direction_values(&self) -> Pair {
        match self {
            Direction::UP(x, y)
            | Direction::RIGHT(x, y)
            | Direction::DOWN(x, y)
            | Direction::LEFT(x, y) => { Pair(*x, *y) }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
struct Pair(i32, i32);

impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Display for Pair
where
    Pair: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pair(a, b) = self;
        write!(f, "({a}, {b})")
    }
}

fn pretty_print_map(map: &Vec<Vec<char>>) {
    map.iter().for_each(|row| {
        row.iter().for_each(|value| {
            print!("{value}")
        });
        println!()
    })
}

pub fn main() {
    let file = File::open("./src/day_06/day_06.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map : Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<char>>());
    }

    let start = map.iter().flatten().position(|c| *c == '^').unwrap();
    let start = Pair((start / map[0].len()) as i32, (start % map[0].len()) as i32);

    // part 1
    let mut visited: HashSet<(Pair, Direction)> = HashSet::new();
    let _ = would_loop(&map, &start, &mut visited);
    let visited = visited.iter().map(|(pos, direction)| *pos).collect::<HashSet<Pair>>();
    println!("number of distinct positions: {}", visited.len());

    // part 2
    let mut placeable_obstacles = 0;
    for pos in &visited {
        let x = pos.0 as usize;
        let y = pos.1 as usize;

        if map[x][y] == '^' {
            continue
        }

        map[x][y] = '#';

        let mut visited: HashSet<(Pair, Direction)> = HashSet::new();
        if would_loop(&map, &start, &mut visited) {
            placeable_obstacles += 1;
        }

        map[x][y] = '.';
    }
    println!("result: {placeable_obstacles}");
}

fn would_loop(map: &Vec<Vec<char>>, start: &Pair, visited: &mut HashSet<(Pair, Direction)>) -> bool {
    let mut direction = Direction::UP(-1, 0);
    let mut current_position = *start;
    loop {
        if visited.contains(&(current_position, direction)) {
            return true;
        }
        visited.insert((current_position, direction));
        let offset = direction.get_direction_values();
        let next_position = current_position + offset;
        if !(0..map.len() as i32).contains(&next_position.0) || !(0..map[0].len() as i32).contains(&next_position.1) {
            return false;
        }
        match map[next_position.0 as usize][next_position.1 as usize] {
            '#' => direction = direction.get_next(),
            '.' | '^' => current_position = next_position,
            other => panic!("Found unknown char '{other}'"),
        }
    }
}
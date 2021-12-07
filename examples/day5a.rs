use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Line(u32, u32, u32, u32);

impl Line {
    pub fn new(numbers: Vec<u32>) -> Line {
        assert_eq!(numbers.len(), 4);

        Line(numbers[0], numbers[1], numbers[2], numbers[3])
    }

    fn inclusive_range(a: u32, b: u32) -> Vec<u32> {
        if a <= b {
            (a..=b).collect()
        } else {
            (b..=a).rev().collect()
        }
    }

    pub fn get_points(&self) -> Vec<(u32, u32)> {
        let xs = Line::inclusive_range(self.0, self.2).into_iter();
        let ys = Line::inclusive_range(self.1, self.3).into_iter();

        xs.zip_longest(ys)
            .map(|e| match e {
                Both(x, y) => (x, y),
                Left(x) => (x, self.1),
                Right(y) => (self.0, y),
            })
            .collect()
    }
}

impl FromStr for Line {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let numbers = s
            .split(" -> ")
            .flat_map(|x| x.split(","))
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let b = Line::new(numbers);
        Ok(b)
    }
}

pub fn main() {
    let input: Vec<_> = include_str!("../day5.txt")
        .lines()
        .map(|n| n.parse::<Line>().unwrap())
        .collect();

    let mut map = HashMap::new();
    for line in input {
        if line.0 == line.2 || line.1 == line.3 {
            for point in line.get_points() {
                *map.entry(point).or_insert(0) += 1
            }
        }
    }

    println!("{}", map.values().filter(|&&x| x > 1).count());
}

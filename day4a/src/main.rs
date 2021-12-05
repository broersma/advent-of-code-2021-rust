use std::num::ParseIntError;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug)]
pub struct Board {
    numbers: Vec<Option<u32>>
}

impl Board {
    pub fn new(numbers: Vec<u32>) -> Board {
        assert_eq!(numbers.len(), 25);

        let numbers = numbers.iter().map(|&o| Some(o)).collect();

        Board { numbers }
    }

    pub fn score(&self) -> u32 {
        self.numbers.iter()
                    .flatten()
                    .sum()
    }

    pub fn is_winning(&self) -> bool {

        let any_row_fully_marked = self.numbers.iter()
                                               .chunks(5)
                                               .into_iter()
                                               .any(|c| {
                                                   c.flatten().count() == 0
                                               });

        let any_column_fully_marked =  self.numbers.iter()
                                                   .enumerate()
                                                   .sorted_by_key(|(i, _)| i % 5)
                                                   .map(|(_, x)| x)
                                                   .chunks(5)
                                                   .into_iter()
                                                   .any(|c| {
                                                       c.flatten().count() == 0
                                                   });

        any_row_fully_marked || any_column_fully_marked
    }

    pub fn mark(&mut self, num: u32) -> bool {
        for elem in &mut self.numbers {
            if *elem == Some(num) {
                *elem = None;
                return true;
            }
        }
        false
    }
}

impl FromStr for Board {

    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let numbers = s.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let b = Board::new(numbers);
        Ok(b)
    }
}


pub fn main() {
    let mut input = include_str!("../../day4.txt")
        .split("\n\n");

    let numbers: Vec<_> = input.next().unwrap().split(",").map(|n| n.parse::<u32>().unwrap()).collect();

    let mut boards: Vec<_> = input.map(|b| b.parse::<Board>().unwrap()).collect();

    for number in numbers {
        for board in &mut boards {
            board.mark(number);

            if board.is_winning() {
                println!("{}", board.score() * number);
                return;
            }
        }
    }
}

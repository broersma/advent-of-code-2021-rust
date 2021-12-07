use itertools::{Either, Itertools};

pub fn main() {
    let input_nums: Vec<_> = include_str!("../../day3.txt")
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    let mut nums = input_nums.clone();

    for i in (0..12).rev() {
        let (zeroes, ones): (Vec<_>, Vec<_>) = nums.iter().partition_map(|r| {
            if r & (1 << i) == 0 {
                Either::Left(r)
            } else {
                Either::Right(r)
            }
        });

        nums = if zeroes.len() > ones.len() {
            zeroes
        } else {
            ones
        };

        if nums.len() == 1 {
            break;
        }
    }

    let oxy = nums[0];

    let mut nums = input_nums;

    for i in (0..12).rev() {
        let (zeroes, ones): (Vec<_>, Vec<_>) = nums.iter().partition_map(|r| {
            if r & (1 << i) == 0 {
                Either::Left(r)
            } else {
                Either::Right(r)
            }
        });

        nums = if zeroes.len() <= ones.len() {
            zeroes
        } else {
            ones
        };

        if nums.len() == 1 {
            break;
        }
    }

    let co2 = nums[0];

    println!("{}", oxy * co2);
}

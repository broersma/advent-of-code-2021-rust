use itertools::Itertools;

pub fn main() {
    println!(
        "{}",
        include_str!("../../day1.txt")
            .lines()
            .map(|i| i.parse::<i32>().unwrap())
            .tuple_windows()
            .filter(|(a,b)| a - b < 0)
            .count()
    );
}

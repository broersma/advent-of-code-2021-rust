pub fn main() {
    let input: Vec<_> = include_str!("../day7.txt")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let min: i32 = *input.iter().min().unwrap();
    let max: i32 = *input.iter().max().unwrap();

    let mut fuel_usage = std::i32::MAX;
    for i in min..=max {
        fuel_usage = fuel_usage.min(input.iter().map(|n| (n - i).abs()).sum());
    }

    println!("{}", fuel_usage);
}

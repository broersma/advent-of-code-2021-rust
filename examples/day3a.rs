pub fn main() {
    let nums = include_str!("../day3.txt")
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap());

    let mut gamma: u32 = 0;

    for i in 0..12 {
        let mut total = 0;

        nums.clone().for_each(|n| {
            if n & (1 << i) == 0 {
                total -= 1;
            } else {
                total += 1;
            }
        });

        if total > 0 {
            gamma += 1 << i;
        }
    }

    let epsilon = (!gamma) & 0b111111111111u32;

    println!("{}", gamma * epsilon);
}

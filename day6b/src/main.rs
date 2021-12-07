pub fn main() {
    let input: Vec<_> = include_str!("../../day6.txt")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut age_pop: [u128; 9] = [0; 9];

    for age in input {
        age_pop[age] += 1;
    }

    for _ in 0..256 {
        let breeders = age_pop[0];
        for i in 0..8 {
            age_pop[i] = age_pop[i + 1];
        }
        age_pop[8] = breeders;
        age_pop[6] += breeders;
    }

    println!("{}", age_pop.iter().sum::<u128>());
}

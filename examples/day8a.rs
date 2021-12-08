pub fn main() {
    let input: Vec<Vec<Vec<_>>> = include_str!("../day8.txt")
        .lines()
        .map(|l| {
            l.split(" | ")
                .map(|l| l.split_whitespace().collect())
                .collect()
        })
        .collect();

    let mut sum = 0;
    for line in input {
        for digit in &line[1] {
            if vec![2, 3, 4, 7].contains(&digit.len()) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}

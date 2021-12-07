pub fn main() {
    let mut depth = 0;
    let mut pos = 0;
    include_str!("../day2.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" ").collect())
        .map(|p: Vec<&str>| (p[0], p[1].parse::<i32>().unwrap()))
        .for_each(|(c, x)| match c {
            "forward" => pos += x,
            "up" => depth -= x,
            "down" => depth += x,
            _ => {}
        });
    println!("{}", depth * pos);
}

use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let numbers: HashSet<u32> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse())
        .flatten()
        .collect();

    let answer1 = numbers
        .iter()
        .filter(|&&x| numbers.contains(&(2020 - x)))
        .map(|x| x * (2020 - x))
        .next();

    println!("#1 = {}", answer1.unwrap());

    for x in numbers.iter().copied() {
        for y in numbers.iter().copied() {
            if x + y > 2020 {
                continue;
            }

            let z = 2020 - x - y;
            if numbers.contains(&z) {
                println!("#2 = {}", x * y * z);
                return Ok(());
            }
        }
    }
    Ok(())
}

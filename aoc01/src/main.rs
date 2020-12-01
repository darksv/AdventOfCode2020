use std::io::BufRead as _;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("input.txt")?;
    let numbers: Vec<u32> = std::io::BufReader::new(file)
        .lines()
        .map(|line| line)
        .flatten()
        .map(|line| line.parse())
        .flatten()
        .collect();

    let answer1 = numbers
        .iter()
        .flat_map(|&x| numbers.iter().map(move |&y| (x, y)))
        .filter(|(x, y)| x + y == 2020)
        .map(|(x, y)| x * y)
        .next();

    if let Some(answer) = answer1 {
        println!("#1 = {}", answer);
    }

    'out:
    for x in numbers.iter().copied() {
        for y in numbers.iter().copied() {
            for z in numbers.iter().copied() {
                if x + y + z == 2020 {
                    println!("#2 = {}", x * y * z);
                    break 'out;
                }
            }
        }
    }
    Ok(())
}

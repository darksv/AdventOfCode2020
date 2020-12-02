fn parse_line(line: &str) -> Option<((usize, usize), char, &str)> {
    let mut parts = line.split(' ');
    let range = parts.next()?;
    let mut range_parts = range.split('-');
    let min = range_parts.next()?.parse().ok()?;
    let max = range_parts.next()?.parse().ok()?;

    let char = parts
        .next()?
        .strip_suffix(':')?
        .chars()
        .next()?;
    let password = parts.next()?;

    Some(((min, max), char, password))
}

fn count_char(password: &str, ch: char) -> usize {
    password.chars().filter(|c| *c == ch).count()
}

fn main() {
    let file: &str = include_str!("../input.txt");
    let items: Vec<_> = file
        .lines()
        .filter_map(|it| parse_line(it))
        .collect();

    let answer1 = items
        .iter()
        .map(|&((min, max), char, password)|
            (min..=max).contains(&count_char(password, char)))
        .filter(|it| *it)
        .count();

    println!("#1 = {}", answer1);

    let answer2 = items
        .iter()
        .map(|&((ia, ib), char, password)| {
            let a =  password.chars().nth(ia - 1);
            let b = password.chars().nth(ib - 1);
            (a == Some(char)) as usize + (b == Some(char)) as usize
        })
        .filter(|it| *it == 1)
        .count();

    println!("#2 = {}", answer2);
}

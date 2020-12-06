fn decode(s: &str) -> u32 {
    let mut bits = 0u32;
    for c in s.chars() {
        bits <<= 1;
        match c {
            'F' | 'L' => bits |= 0,
            'B' | 'R' => bits |= 1,
            _ => unimplemented!(),
        }
    }
    bits
}

fn main() {
    let input = include_str!("../input.txt");
    let mut seats: Vec<_> = input.lines().map(decode).collect();
    seats.sort_unstable();

    let answer1 = seats.iter().copied().last();
    println!("#1 = {}", answer1.unwrap());

    let answer2 = seats
        .windows(2)
        .filter_map(|pair| if pair[1] == pair[0] + 2 { Some(pair[0] + 1) } else { None })
        .next();
    println!("#2 = {}", answer2.unwrap());
}

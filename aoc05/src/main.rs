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
    let seats: Vec<_> = input.lines().map(decode).collect();
    let answer1 = seats.iter().copied().max();
    println!("#1 = {}", answer1.unwrap());

    let mut available_seats = [true; 8 * 128];
    for &id in seats.iter() {
        available_seats[id as usize] = false;
    }

    let answer2 = available_seats
        .iter()
        .copied()
        .enumerate()
        .skip_while(|(id, seat)| *seat)
        .skip_while(|(id, seat)| !*seat)
        .map(|it| it.0)
        .next();

    println!("#2 = {}", answer2.unwrap());
}

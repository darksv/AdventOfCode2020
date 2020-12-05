fn bipart(s: &str, max: u32) -> u32 {
    let mut lower = 0;
    let mut upper = max;

    for c in s.chars() {
        match c {
            'F' | 'L' => {
                upper = (lower + upper + 1) / 2 - 1;
            }
            'B' | 'R' => {
                lower = (lower + upper + 1) / 2;
            }
            _ => unimplemented!(),
        }
    }
    assert_eq!(lower, upper);
    lower
}

fn find_place(s: &str) -> (u32, u32) {
    (bipart(&s[..7], 127), bipart(&s[7..], 7))
}

fn main() {
    let input = include_str!("../input.txt");
    let seats: Vec<_> = input.lines().map(find_place).collect();
    let answer1 = seats.iter().map(|&(row, column)| row * 8 + column).max();
    println!("#1 = {}", answer1.unwrap());

    let mut available_seats = [true; 8 * 128];
    for &(row, column) in seats.iter() {
        available_seats[row as usize * 8 + column as usize] = false;
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

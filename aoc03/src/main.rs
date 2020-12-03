#[derive(Copy, Clone, Eq, PartialEq)]
enum Field {
    Open,
    Tree,
}

fn parse_map(map: &str) -> Option<Vec<Vec<Field>>> {
    map
        .lines()
        .map(|line| -> Option<_> {
            line
                .chars()
                .map(|it| match it {
                    '#' => Some(Field::Tree),
                    '.' => Some(Field::Open),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn find_trees(map: &[Vec<Field>], dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < map.len() - 1 {
        x += dx;
        y += dy;
        if map[y][x % map[0].len()] == Field::Tree {
            trees += 1;
        }
    }

    trees
}

fn main() {
    let map = parse_map(include_str!("../input.txt"))
        .expect("invalid map");
    let answer1 = find_trees(&map, 3, 1);
    println!("#1 = {}", answer1);

    let answer2: usize = [(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(dx, dy)| find_trees(&map, dx, dy))
        .product();
    println!("#2 = {}", answer2);
}
#[derive(Copy, Clone, Eq, PartialEq)]
enum Field {
    Open,
    Tree,
}

struct Map {
    stride: usize,
    fields: Vec<Field>,
}

impl Map {
    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<Field> {
        self.fields.get(y * self.stride + (x % self.stride)).copied()
    }

    #[inline]
    fn height(&self) -> usize {
        self.fields.len() / self.stride
    }
}

fn parse_map(map: &str) -> Option<Map> {
    let stride = map.lines().next().unwrap_or("").len();
    let fields: Option<Vec<_>> = map
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| {
            match c {
                '#' => Some(Field::Tree),
                '.' => Some(Field::Open),
                _ => None,
            }
        })
        .collect();
    Some(Map {
        stride,
        fields: fields?,
    })
}

fn find_trees(map: &Map, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < map.height() - 1 {
        x += dx;
        y += dy;
        if map.get(x, y) == Some(Field::Tree) {
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
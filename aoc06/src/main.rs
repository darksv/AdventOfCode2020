use std::collections::HashSet;

#[derive(Debug)]
struct Group {
    answered_with_true: Vec<HashSet<u8>>,
    group_id: u32,
}

fn main() {
    let groups: Vec<Group> = include_str!("../input.txt")
        .split("\n\n")
        .enumerate()
        .map(|(group_id, persons)| {
            Group {
                group_id: group_id as u32,
                answered_with_true: persons
                    .split('\n')
                    .filter(|p| !p.is_empty())
                    .map(|p| p.bytes().map(|a| a - b'a').collect())
                    .collect(),
            }
        })
        .collect();

    let answer1: usize = groups
        .iter()
        .map(|g| {
            g.answered_with_true
                .iter()
                .flatten()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!("#1 = {}", answer1);

    let answer2: usize = groups
        .iter()
        .map(|g| {
            fold_first(g.answered_with_true.iter().cloned(),
                       |a, b| a.intersection(&b)
                           .copied()
                           .collect())
                .unwrap()
                .len()
        })
        .sum();
    println!("#2 = {}", answer2);
}

fn fold_first<F, I>(mut it: I, f: F) -> Option<I::Item>
    where
        I: Sized,
        I: Iterator,
        F: FnMut(I::Item, I::Item) -> I::Item,
{
    let first = it.next()?;
    Some(it.fold(first, f))
}

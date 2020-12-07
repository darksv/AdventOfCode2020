use std::collections::HashMap;

#[derive(Debug)]
struct Rule<'a> {
    name: &'a str,
    inside: Vec<(u32, &'a str)>,
}

fn parse_line(s: &str) -> Option<Rule> {
    let mut a = s.strip_suffix('.')?.split(" bags contain ");
    let first = a.next()?;
    let internal = a.next()?;

    let mut rule = Rule {
        name: first,
        inside: vec![],
    };

    for internal in internal.split(", ") {
        let mut parts = internal.splitn(2, ' ');
        let count = parts.next()?;
        let rest = parts.next()?;
        let name = rest.rsplitn(2, ' ').nth(1)?;

        rule.inside.push((count.parse().ok()?, name));
    }

    Some(rule)
}

fn main() {
    let input = include_str!("../input.txt");
    let bags: HashMap<_, Vec<_>> = input
        .lines()
        .filter_map(parse_line)
        .map(|rule| (rule.name, rule.inside.clone()))
        .collect();

    let answer1 = bags
        .values()
        .map(|bags_inside| {
            let mut search_queue = bags_inside.clone();
            let mut has_shiny_gold = false;
            while let Some((_count, name)) = search_queue.pop() {
                if name == "shiny gold" {
                    has_shiny_gold = true;
                    break;
                }

                if let Some(v) = bags.get(name) {
                    search_queue.extend(v.iter());
                }
            }
            has_shiny_gold
        })
        .filter(|it| *it)
        .count();
    println!("#1 = {}", answer1);

    let mut answer2: u32 = 0;
    let mut search_queue = bags["shiny gold"].clone();
    while let Some((quantity, name)) = search_queue.pop() {
        answer2 += quantity;
        if let Some(v) = bags.get(name) {
            for (q, name) in v.iter().copied() {
                search_queue.push((q * quantity, name));
            }
        }
    }
    println!("#2 = {}", answer2);
}

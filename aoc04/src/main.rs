fn validate_number(s: &str, min: u32, max: u32) -> bool {
    match s.parse() {
        Ok(x) => (min..=max).contains(&x),
        Err(_) => false,
    }
}

fn validate_strict(s: &str) -> bool {
    for pair in s.split_whitespace().filter(|it| !it.is_empty()) {
        let mut parts = pair.split(':');
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");

        let valid = match key {
            "byr" => validate_number(value, 1920, 2002),
            "iyr" => validate_number(value, 2010, 2020),
            "eyr" => validate_number(value, 2020, 2030),
            "hgt" => {
                if let Some(val) = value.strip_suffix("cm") {
                    validate_number(val, 150, 193)
                } else if let Some(val) = value.strip_suffix("in") {
                    validate_number(val, 59, 76)
                } else {
                    false
                }
            }
            "hcl" => value.starts_with("#") && value.len() == 7 && value[1..].chars().all(|c| c.is_ascii_hexdigit()),
            "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
            "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
            "cid" => true,
            _ => false,
        };

        if !valid {
            return false;
        }
    }

    true
}

fn validate(s: &str) -> bool {
    let mut fields = 0;
    for pair in s.split_whitespace().filter(|it| !it.is_empty()) {
        let key = pair.split(':').next().unwrap_or("");
        match key {
            "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => fields += 1,
            "cid" => {},
            _ => return false,
        }
    }
    fields == 7
}

fn main() {
    let input = include_str!("../input.txt");

    let answer1 = input
        .split("\n\n")
        .filter(|it| validate(it))
        .count();
    println!("#1 = {}", answer1);

    let answer2 = input
        .split("\n\n")
        .filter(|it| validate(it))
        .filter(|it| validate_strict(it))
        .count();
    println!("#2 = {}", answer2);
}

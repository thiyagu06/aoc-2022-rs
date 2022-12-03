fn main() {
    println!("{:}", star1());
    println!("{:}", star2());
}

fn star1() -> i32 {
    let data = include_str!("./input.txt");
    let rucksacks: Vec<&str> = data.lines().collect();
    return rucksacks.iter()
        .filter_map(|sack| { return Some(common_term(sack)); })
        .map(priority)
        .sum();
}

fn star2() -> i32 {
    let data = include_str!("./input.txt");
    let rucksacks: Vec<&str> = data.lines().collect();
    let total_priority = rucksacks
        .chunks_exact(3)
        .filter_map(|sack| { return Some(common_term3(sack)); })
        .map(|common| { return priority(common.unwrap()); })
        .sum();
    return total_priority;
}

fn common_term3(sacks: &[&str]) -> Option<char> {
    let (pile1, pile2, pile3) = (sacks[0], sacks[1], sacks[2]);
    return pile1.chars()
        .find(|item| pile2.contains(*item) & pile3.contains(*item));
}


fn common_term(line: &str) -> char {
    let (left, right) = line.split_at(line.chars().count() / 2);
    return left.chars().find(|c| right.contains(*c)).unwrap();
}

fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        (c as u8 - 'a' as u8 + 1) as i32
    } else {
        (c as u8 - 'A' as u8 + 27) as i32
    }
}
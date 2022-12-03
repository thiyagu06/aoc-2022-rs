fn main() {
    println!("Star1::{:}", star1());
    println!("Star2::{:}", star2());
}

fn star1() -> u32 {
    let res = include_str!("./input.txt")
        .lines()
        .map(|l| match l.chars().collect::<Vec<char>>().as_slice() {
            ['A', _, 'X'] => 4,
            ['B', _, 'Y'] => 5,
            ['C', _, 'Z'] => 6,
            ['C', _, 'X'] => 7,
            ['A', _, 'Y'] => 8,
            ['B', _, 'Z'] => 9,
            [_, _, c] => match c {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0,
            },
            _ => 0,
        })
        .sum::<u32>();
    return res;
}

fn star2() -> u32 {
    return include_str!("./input.txt")
        .lines()
        .map(|l| match l.chars().collect::<Vec<char>>().as_slice() {
            [a, _, 'X'] => fnl(a),
            [a, _, 'Y'] => fnd(a),
            [a, _, 'Z'] => fnw(a),
            _ => 0,
        })
        .sum::<u32>();
}

fn fnl(c: &char) -> u32 {
    match c {
        'A' => 0 + 3,
        'B' => 0 + 1,
        'C' => 0 + 2,
        _ => 0,
    }
}

fn fnd(c: &char) -> u32 {
    match c {
        'A' => 3 + 1,
        'B' => 3 + 2,
        'C' => 3 + 3,
        _ => 0,
    }
}

fn fnw(c: &char) -> u32 {
    match c {
        'A' => 6 + 2,
        'B' => 6 + 3,
        'C' => 6 + 1,
        _ => 0,
    }
}
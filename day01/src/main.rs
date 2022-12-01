fn main() {
    println!("Star1::{:}", star1());
    println!("Star2::{:}", star2());
}

fn star1() -> usize {
    let res = include_str!("./input.txt")
        .trim()
        .split("\n\n")
        .map(|x| return x.split("\n").map(|x| x.parse::<usize>().unwrap()))
        .map(|x| x.sum::<usize>())
        .max();

    return res.unwrap()
}

fn star2() -> usize {
    let mut res = include_str!("./input.txt")
        .trim()
        .split("\n\n")
        .map(|x| return x.split("\n").map(|x| x.parse::<usize>().unwrap()))
        .map(|x| x.sum::<usize>())
        .collect::<Vec<_>>();
    res.sort_by(|a, b| a.cmp(b));
    return res.iter().take(3).sum::<usize>()
}
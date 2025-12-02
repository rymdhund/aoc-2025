
use std::fs;
use std::str;

fn parse(filename: &str) -> Vec<(i64, i64)> {
    fs::read_to_string(filename).unwrap().trim().split(",").map(|range| {
        let (a, b) = range.split_once("-").unwrap();
        (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
    }).collect()
}

fn main() {
    let inp = parse("inputs/day2.txt");

    println!("solution1: {}", solve1(&inp));
    println!("solution2: {}", solve2(&inp));
}

fn solve1(input: &Vec<(i64, i64)>) -> i64 {
    input.iter().map(|&(a, b)| {
        (a..(b+1)).filter(|x| invalid1(*x)).sum::<i64>()
    }).sum()
}

fn solve2(input: &Vec<(i64, i64)>) -> i64 {
    input.iter().map(|&(a, b)| {
        (a..(b+1)).filter(|x| invalid2(*x)).sum::<i64>()
    }).sum()
}

#[test]
fn test_invalid1() {
    assert!(invalid1(11));
    assert!(!invalid1(111));
    assert!(invalid1(1111));
}

fn invalid1(x: i64) -> bool {
    let binding = x.to_string();
    let s = binding.as_bytes();
    let half = s.len()/2;
    let p1 = &s[..half];
    let p2 = &s[half..];
    p1 == p2
}

#[test]
fn test_invalid2() {
    assert!(invalid2(11));
    assert!(invalid2(111));
    assert!(invalid2(1111));
    assert!(!invalid2(1211));
}

fn invalid2(x: i64) -> bool {
    let s = x.to_string().into_bytes();

    'outer: for len in 1..(s.len()/2 + 1) {
        if s.len() % len != 0 {
            continue;
        }
        let p1 = &s[..len];
        
        for i in 1..(s.len() / len) {
            let p2 = &s[(i * len)..((i+1) * len)];
            if p1 != p2 {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

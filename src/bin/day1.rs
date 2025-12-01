use std::fs;

fn parse(file: &str) -> Vec<(char, i32)> {
    fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        let c = line.chars().next().unwrap();
        let rest = line[1..].parse::<i32>().unwrap();
        (c, rest)
    }).collect()
}

fn main() {
    let input = parse("inputs/day1.txt");
    
    println!("solution1: {}", solve1(&input));
    println!("solution2: {}", solve2(&input));
}

fn solve1(input: &Vec<(char, i32)>) -> i32 {
    let mut zeroes = 0;
    let mut cur = 50;
    for &(d, n) in input {
        if d == 'L' {
            cur = (cur + 100 - n) % 100;
        } else {
            cur = (cur + n) % 100;
        }
        if cur == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

fn solve2(input: &Vec<(char, i32)>) -> i32 {
    let mut zeroes = 0;
    let mut cur = 50;
    for &(d, n) in input {
        if d == 'L' {
            cur = (100-cur) % 100;
        }

        cur += n;
        zeroes += cur / 100;
        cur = cur % 100;

        if d == 'L' {
            cur = (100-cur) % 100;
        }
    }
    zeroes
}
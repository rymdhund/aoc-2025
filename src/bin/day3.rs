use std::fs;

fn parse(filename: &str) -> Vec<Vec<u64>> {
    fs::read_to_string(filename).unwrap().trim().lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()
    }).collect()
}

fn main() {
    let input = parse("inputs/day3_ex.txt");

    println!("solution1: {}", solve(&input,2));
    println!("solution2: {}", solve(&input,12));
}

fn solve(inp: &Vec<Vec<u64>>, n: usize) -> u64 {
    inp.iter().map(|row| {
        let mut start = 0;
        let mut acc = 0;
        for left in (0..n).rev() {
            let (pos, max) = max_with_pos(&row[start..(row.len()-left)]);
            start += pos + 1;
            acc = acc * 10 + max;
        }
        acc
    }).sum()
}


fn max_with_pos(v: &[u64]) -> (usize, u64) {
    let mut max = 0u64;
    let mut pos = 0usize;
    for (i, &el) in v.iter().enumerate() {
        if el > max {
            max = el;
            pos = i;
        }
    }
    (pos, max)
}
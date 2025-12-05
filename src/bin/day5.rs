use std::fs;

use itertools::Itertools;

fn parse(filename: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let s = fs::read_to_string(filename).unwrap();
    let (a, b) = s.trim().split_once("\n\n").unwrap();
    let ranges = a.lines().map(|line| {
        let parts = line.split("-").map(|v| v.parse::<u64>().unwrap()).collect_vec();
        (parts[0], parts[1])
    }).collect_vec();

    let values = b.lines().map(|line| line.parse::<u64>().unwrap()).collect_vec();

    (ranges, values)
}

fn main() {
    let inp = parse("inputs/day5_ex.txt");

    println!("solution1: {}", solve1(&inp));
    println!("solution2: {}", solve2(&inp.0));
}

fn solve1(inp: &(Vec<(u64,u64)>, Vec<u64>)) -> usize {
    let (ranges, values) = inp;

    values.iter().filter(|&v| {
        ranges.iter().any(|(a, b)| v >= a && v <= b)
    }).count()
}

// sort and then collapse overlapping ranges
fn solve2(ranges: &Vec<(u64,u64)>) -> u64 {
    let mut ranges = ranges.clone();
    ranges.sort_by(|&a, b| a.0.cmp(&b.0));

    let mut tot = 0;
    let mut cur = ranges[0];

    for r in ranges {
        if r.0 > cur.1 {
            tot += len(cur);
            cur = r;
        } else if r.1 > cur.1 {
            cur = (cur.0, r.1);
        }
    }
    tot + len(cur)
}

fn len(r: (u64, u64)) -> u64 {
    r.1 - r.0 + 1
}
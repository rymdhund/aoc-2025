use std::fs;
use itertools::Itertools;
use aoc2025::coord::{coord_iter, CoordMap, Coord};

fn parse(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename).unwrap().lines().map(|line|
        line.chars().collect_vec()
    ).collect()
}

fn main() {
    let inp = parse("inputs/day4_ex.txt");

    println!("solution1: {}", solve1(&inp));
    println!("solution2: {}", solve2(&inp));
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    accessible(map).len()
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut map = map.clone();

    let mut removed = 0;
    loop {
        let a = accessible(&map);
        if a.is_empty() {
            return removed;
        }
        a.iter().for_each(|p| map[p.y_u()][p.x_u()] = 'x');
        removed += a.len();
    }
}

fn accessible(map: &Vec<Vec<char>>) -> Vec<Coord> {
    coord_iter(map).filter(|&p| {
        if *map.at(p) != '@' {
            return false;
        }

        // we include the center in the count
        let cnt = (-1..2).flat_map(|x| {
            (-1..2).filter(move |y| {
                let p1 = p + Coord::new(x, *y);
                map.contains(p1) && *map.at(p1) == '@'
            })
        }).count();

        cnt <= 4
    }).collect()
}
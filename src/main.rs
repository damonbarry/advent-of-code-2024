use itertools::Itertools;
use std::fs;

fn main() {
    calculate_left_right_list_distance();
}

fn calculate_left_right_list_distance() {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    let lines = input.lines();
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines
        .map(|l| -> (u64, u64) {
            l.split_ascii_whitespace()
                .map(|id| id.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();
    let sum = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| r.abs_diff(l))
        .sum::<u64>();

    println!("The total distance between left and right lists is {}", sum,);
}

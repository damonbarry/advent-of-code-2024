use itertools::Itertools;
use std::fs;

fn main() {
    calculate_left_right_list_distance();
    calculate_left_right_list_similarity_score();
    sum_safe_reports();
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

    println!("The total distance between left and right lists is {}", sum);
}

fn calculate_left_right_list_similarity_score() {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    let lines = input.lines();
    let (left, right): (Vec<_>, Vec<_>) = lines
        .map(|l| -> (u64, u64) {
            l.split_ascii_whitespace()
                .map(|id| id.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    let right_id_counts = right.into_iter().counts();
    let similarity_score = left
        .into_iter()
        .map(|id| id * *right_id_counts.get(&id).unwrap_or(&0) as u64)
        .sum::<u64>();

    println!(
        "The similarity score of the left and right lists is {}",
        similarity_score
    );
}

fn sum_safe_reports() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let lines = input.lines();
    let reports = lines.map(|l| {
        let levels: Vec<_> = l
            .split_ascii_whitespace()
            .map(|id| id.parse::<u64>().unwrap())
            .collect();
        // A report is safe if:
        // 1. All levels are in increasing or decreasing order
        // 2. A level differs from its predecessor by at least one and at most three
        let safe = (levels.windows(2).all(|w| w[0] < w[1])
            || levels.windows(2).all(|w| w[0] > w[1]))
            && levels.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3);
        safe
    });

    println!(
        "The number of safe reports is {}",
        reports.filter(|&is_safe| is_safe).count()
    );
}

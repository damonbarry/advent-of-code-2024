use itertools::Itertools;
use std::fs;

fn main() {
    calculate_left_right_list_distance();
    calculate_left_right_list_similarity_score();
    sum_safe_reports();
    sum_safe_reports_with_problem_dampener();
    sum_uncorrupted_mul_instructions();
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

fn get_reports(input: &str) -> Vec<Vec<u64>> {
    let input = fs::read_to_string(input).unwrap();
    let lines = input.lines();
    lines
        .map(|l| -> Vec<u64> {
            l.split_ascii_whitespace()
                .map(|id| id.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(levels: &[u64]) -> bool {
    // A report is safe if:
    // 1. All levels are in increasing or decreasing order
    // 2. A level differs from its predecessor by at least one and at most three
    (levels.windows(2).all(|w| w[0] < w[1]) || levels.windows(2).all(|w| w[0] > w[1]))
        && levels.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3)
}

fn sum_safe_reports() {
    let safe_reports = get_reports("input/day2.txt")
        .into_iter()
        .filter_map(|levels| {
            if is_report_safe(levels.as_slice()) {
                Some(levels)
            } else {
                None
            }
        });

    println!("The number of safe reports is {}", safe_reports.count());
}

fn sum_safe_reports_with_problem_dampener() {
    let reports = get_reports("input/day2.txt");
    let safe_reports = reports.into_iter().filter_map(|levels| {
        if is_report_safe(levels.as_slice()) {
            Some(levels)
        } else {
            for i in 0..levels.len() {
                let mut levels = levels.clone();
                levels.remove(i);
                if is_report_safe(levels.as_slice()) {
                    return Some(levels);
                }
            }

            None
        }
    });

    println!("The number of safe reports is {}", safe_reports.count());
}

fn sum_uncorrupted_mul_instructions() {
    let input = fs::read_to_string("input/day3.txt").unwrap();
    let sum: u64 = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .filter_map(|(i, w)| {
            if w[0] == b'm' && w[1] == b'u' && w[2] == b'l' && w[3] == b'(' {
                let argp = i + 4;
                let mut j = argp;

                while input.as_bytes()[j].is_ascii_digit() {
                    j += 1;
                }

                if j - argp == 0 || input.as_bytes()[j] != b',' {
                    return None;
                }

                let arg1 = input[argp..j].parse::<u64>().unwrap();
                let argp = j + 1;
                j = argp;

                while input.as_bytes()[j].is_ascii_digit() {
                    j += 1;
                }

                if j - argp == 0 || input.as_bytes()[j] != b')' {
                    return None;
                }

                let arg2 = input[argp..j].parse::<u64>().unwrap();
                Some(arg1 * arg2)
            } else {
                None
            }
        })
        .sum();

    println!("The sum of uncorrupted mul instructions is {}", sum);
}

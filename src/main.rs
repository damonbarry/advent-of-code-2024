use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() {
    calculate_left_right_list_distance();
    calculate_left_right_list_similarity_score();
    sum_safe_reports();
    sum_safe_reports_with_problem_dampener();
    sum_uncorrupted_mul_instructions();
    sum_enabled_multiplications();
    sum_xmas_words();
    sum_mas_in_the_shape_of_an_x();
    sum_middle_page_numbers_in_correctly_ordered_updates();
    sum_middle_page_numbers_in_incorrectly_ordered_updates();
    sum_visited_guard_positions();
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

fn sum_enabled_multiplications() {
    let input = fs::read_to_string("input/day3.txt").unwrap();
    let mut sum = 0;
    let mut mul_enabled = true;

    for (i, w) in input.as_bytes().windows(7).enumerate() {
        if w[0] == b'd' && w[1] == b'o' && w[2] == b'(' && w[3] == b')' {
            // parse do() instruction
            mul_enabled = true;
        } else if w[0] == b'd'
            && w[1] == b'o'
            && w[2] == b'n'
            && w[3] == b'\''
            && w[4] == b't'
            && w[5] == b'('
            && w[6] == b')'
        {
            // parse don't() instruction
            mul_enabled = false;
        } else if mul_enabled && w[0] == b'm' && w[1] == b'u' && w[2] == b'l' && w[3] == b'(' {
            // parse mul(x,y) instruction
            let argp = i + 4;
            input[argp..].find(',').and_then(|j| {
                input[argp..argp + j].parse::<u64>().ok().and_then(|arg1| {
                    let argp = argp + j + 1;
                    input[argp..].find(')').and_then(|j| {
                        input[argp..argp + j].parse::<u64>().ok().map(|arg2| {
                            sum += arg1 * arg2;
                        })
                    })
                })
            });
        }
    }

    println!("The sum of enabled multiplications is {}", sum);
}

fn sum_xmas_words() {
    let mut xmas_words = 0;

    // input file can be visualized as a 2D grid of characters
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    // visit each cell in the grid looking for an 'X' character
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'X' {
                // Found an 'X', now search in all directions for 'M', 'A', 'S'

                // Search for horizontal right
                if j < line.len() - 3 && &line[j + 1..j + 4] == "MAS" {
                    xmas_words += 1;
                }

                // Search for horizontal left
                if j >= 3 && &line[j - 3..j] == "SAM" {
                    xmas_words += 1;
                }

                // Search vertical descending
                if i < lines.len() - 3
                    && lines[i + 1].chars().nth(j).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j).unwrap() == 'S'
                {
                    xmas_words += 1;
                }

                // Search vertical ascending
                if i >= 3
                    && lines[i - 1].chars().nth(j).unwrap() == 'M'
                    && lines[i - 2].chars().nth(j).unwrap() == 'A'
                    && lines[i - 3].chars().nth(j).unwrap() == 'S'
                {
                    xmas_words += 1;
                }

                // Search diagonal descending right
                if i < lines.len() - 3
                    && j < line.len() - 3
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j + 2).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j + 3).unwrap() == 'S'
                {
                    xmas_words += 1;
                }

                // Search diagonal descending left
                if i < lines.len() - 3
                    && j >= 3
                    && lines[i + 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j - 2).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j - 3).unwrap() == 'S'
                {
                    xmas_words += 1;
                }

                // Search diagonal ascending right
                if i >= 3
                    && j < line.len() - 3
                    && lines[i - 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i - 2].chars().nth(j + 2).unwrap() == 'A'
                    && lines[i - 3].chars().nth(j + 3).unwrap() == 'S'
                {
                    xmas_words += 1;
                }

                // Search diagonal ascending left
                if i >= 3
                    && j >= 3
                    && lines[i - 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i - 2].chars().nth(j - 2).unwrap() == 'A'
                    && lines[i - 3].chars().nth(j - 3).unwrap() == 'S'
                {
                    xmas_words += 1;
                }
            }
        }
    }

    println!("The sum of XMAS words is {}", xmas_words);
}

fn sum_mas_in_the_shape_of_an_x() {
    let mut sum_x_mas = 0;

    // input file can be visualized as a 2D grid of characters
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    // visit each cell in the grid looking for an 'A' character
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'A' && i >= 1 && j >= 1 && i < lines.len() - 1 && j < line.len() - 1 {
                // Found an 'A', now search for any of the following patterns:
                // (top) M M  (left) M S  (bottom) S S  (right) S M
                //        A           A             A            A
                //       S S         M S           M M          S M

                // Search for top
                if lines[i - 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i - 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i + 1].chars().nth(j - 1).unwrap() == 'S'
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'S'
                {
                    sum_x_mas += 1;
                }

                // Search for left
                if lines[i - 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i + 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i - 1].chars().nth(j + 1).unwrap() == 'S'
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'S'
                {
                    sum_x_mas += 1;
                }

                // Search for bottom
                if lines[i + 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i - 1].chars().nth(j - 1).unwrap() == 'S'
                    && lines[i - 1].chars().nth(j + 1).unwrap() == 'S'
                {
                    sum_x_mas += 1;
                }

                // Search for right
                if lines[i - 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i - 1].chars().nth(j - 1).unwrap() == 'S'
                    && lines[i + 1].chars().nth(j - 1).unwrap() == 'S'
                {
                    sum_x_mas += 1;
                }
            }
        }
    }

    println!("The sum of X-MAS is {}", sum_x_mas);
}

#[derive(PartialEq)]
enum UpdateTypes {
    OnlyCorrect,
    OnlyFixed,
}

fn sum_middle_page_numbers_in_ordered_updates(update_type: UpdateTypes) {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let lines = input.lines();
    let mut page_ordering_rules = true;
    let mut page_order: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut keep_updates: Vec<Vec<u64>> = Vec::new();

    for l in lines {
        if l.is_empty() {
            // found delimiter between page ordering rules and updates
            page_ordering_rules = false;
        } else if page_ordering_rules {
            // parse page ordering rules, in the form "X|Y" where page #X comes before page #Y
            let pages: Vec<u64> = l
                .split("|")
                .map(|part| part.parse::<u64>().unwrap())
                .collect();
            assert!(pages.len() == 2);

            // Build a hash map with page numbers as keys, and with the list of page numbers that
            // come before the key page number as values.
            if !page_order.contains_key(&pages[1]) {
                page_order.insert(pages[1], vec![pages[0]]);
            } else {
                page_order.get_mut(&pages[1]).unwrap().push(pages[0]);
            }
        } else {
            // Parse updates in the form "X, Y, ..." where X, Y, ... are page numbers
            let parsed_updates: Vec<u64> = l
                .split(',')
                .map(|part| part.parse::<u64>().unwrap())
                .collect();

            // Build a new update that adheres to page ordering rules using the page numbers from
            // the parsed update. Discard the parsed update if it doesn't match what we build here.
            let mut new_updates = Vec::new();
            for u in &parsed_updates {
                if new_updates.len() == 0 {
                    new_updates.push(*u);
                } else {
                    let mut inserted = false;
                    for (i, new_u) in new_updates.iter().enumerate() {
                        if page_order.get(&new_u).unwrap().contains(&u) {
                            new_updates.insert(i, *u);
                            inserted = true;
                            break;
                        }
                    }

                    if !inserted {
                        new_updates.push(*u);
                    }
                }
            }

            assert!(parsed_updates.len() == new_updates.len());
            if update_type == UpdateTypes::OnlyCorrect && parsed_updates == new_updates {
                keep_updates.push(parsed_updates);
            } else if update_type == UpdateTypes::OnlyFixed && parsed_updates != new_updates {
                keep_updates.push(new_updates);
            }
        }
    }

    let sum: u64 = keep_updates
        .iter()
        .map(|u| u.get(u.len() / 2).unwrap())
        .sum();
    println!(
        "The sum of middle page numbers in {} ordered updates is {}",
        if update_type == UpdateTypes::OnlyCorrect {
            "correctly"
        } else {
            "(corrected) incorrectly"
        },
        sum
    );
}

fn sum_middle_page_numbers_in_correctly_ordered_updates() {
    sum_middle_page_numbers_in_ordered_updates(UpdateTypes::OnlyCorrect);
}

fn sum_middle_page_numbers_in_incorrectly_ordered_updates() {
    sum_middle_page_numbers_in_ordered_updates(UpdateTypes::OnlyFixed);
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn sum_visited_guard_positions() {
    let input = fs::read_to_string("input/day6.txt").unwrap();
    let lab_map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    // Find the initial position of the guard
    let find_guard = || -> Option<(usize, usize)> {
        for i in 0..lab_map.len() {
            for j in 0..lab_map[i].len() {
                if lab_map[i][j] == '^' {
                    return Some((i, j));
                }
            }
        }

        None
    };

    let (mut i, mut j) = find_guard().unwrap();

    let mut dir = Direction::Up;
    let mut visits: Vec<(usize, usize)> = vec![(i, j)];

    loop {
        let mut new_i = i;
        let mut new_j = j;

        // the guard attempts to move one step forward 
        match dir {
            Direction::Up => {
                if i > 0 {
                    new_i -= 1;
                }
            }
            Direction::Right => {
                if j < lab_map[i].len() - 1 {
                    new_j += 1;
                }
            }
            Direction::Down => {
                if i < lab_map.len() - 1 {
                    new_i += 1;
                }
            }
            Direction::Left => {
                if j > 0 {
                    new_j -= 1;
                }
            }
        }

        if i == new_i && j == new_j {
            // the gaurd exited the bounds of the lab
            break;
        }

        if lab_map[new_i][new_j] == '#' {
            // the guard encountered an obstacle, turns right 90 degrees
            match dir {
                Direction::Up => dir = Direction::Right,
                Direction::Right => dir = Direction::Down,
                Direction::Down => dir = Direction::Left,
                Direction::Left => dir = Direction::Up,
            }
        } else {
            visits.push((new_i, new_j));
            i = new_i;
            j = new_j;
        }
    }

    // remove duplicates to get distinct positions the guard visited
    visits.sort();
    visits.dedup();

    println!("The sum of visited guard positions is {}", visits.len());
}

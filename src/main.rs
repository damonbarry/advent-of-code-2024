use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, fs};

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
    // day 6 part 2 is slow; disable for now
    // _sum_candidate_obstacle_positions();
    sum_bridge_calibrations_from_two_operations();
    // _sum_bridge_calibrations_from_three_operations();
    // day 8 part 1 is slow; disable for now
    // _sum_unique_antinode_locations();
    // day 8 part 2 is slow; disable for now
    // _sum_unique_antinode_locations_accounting_for_resonant_harmonics();
    compute_filesystem_checksum_following_block_compaction();
    compute_filesystem_checksum_following_file_compaction();
    sum_scores_of_all_trailheads_on_topo_map();
    sum_ratings_of_all_trailheads_on_topo_map();
    sum_stones_after_25_blinks();
    sum_stones_after_75_blinks();
    total_fencing_price_for_all_regions();
}

fn calculate_left_right_list_distance() {
    let input = fs::read_to_string("src/input/day1.txt").unwrap();
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
    let input = fs::read_to_string("src/input/day1.txt").unwrap();
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
    let safe_reports = get_reports("src/input/day2.txt")
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
    let reports = get_reports("src/input/day2.txt");
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
    let input = fs::read_to_string("src/input/day3.txt").unwrap();
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
    let input = fs::read_to_string("src/input/day3.txt").unwrap();
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
    let input = fs::read_to_string("src/input/day4.txt").unwrap();
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
    let input = fs::read_to_string("src/input/day4.txt").unwrap();
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
    let input = fs::read_to_string("src/input/day5.txt").unwrap();
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

fn find_guard(lab_map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..lab_map.len() {
        for j in 0..lab_map[i].len() {
            if lab_map[i][j] == '^' {
                return Some((i, j));
            }
        }
    }

    None
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum PatrolProtocolOutcome {
    Move((usize, usize)),
    Turn(Direction),
    Exit,
}

fn patrol_protocol(
    i: &usize,
    j: &usize,
    dir: &Direction,
    lab_map: &Vec<Vec<char>>,
) -> PatrolProtocolOutcome {
    let mut new_i = *i;
    let mut new_j = *j;

    // the guard attempts to move one step forward
    match *dir {
        Direction::Up => {
            if *i > 0 {
                new_i -= 1;
            }
        }
        Direction::Right => {
            if *j < lab_map[*i].len() - 1 {
                new_j += 1;
            }
        }
        Direction::Down => {
            if *i < lab_map.len() - 1 {
                new_i += 1;
            }
        }
        Direction::Left => {
            if *j > 0 {
                new_j -= 1;
            }
        }
    }

    if *i == new_i && *j == new_j {
        // the gaurd exits the bounds of the lab
        return PatrolProtocolOutcome::Exit;
    }

    if lab_map[new_i][new_j] == '#' {
        // the guard encounters an obstacle, turns right 90 degrees
        let next_direction = || match *dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };

        return PatrolProtocolOutcome::Turn(next_direction());
    } else {
        return PatrolProtocolOutcome::Move((new_i, new_j));
    }
}

enum GuardRouteOutcome {
    Positions(Vec<GuardPosition>),
    LoopDetected,
}

#[derive(Debug, Copy, Clone)]
struct GuardPosition {
    coordinates: (usize, usize),
    direction: Direction,
}

impl GuardPosition {
    pub fn new(coords: (usize, usize), dir: Direction) -> Self {
        GuardPosition {
            coordinates: coords,
            direction: dir,
        }
    }
}

fn _print_guard_route(lab_map: &Vec<Vec<char>>, route: &Vec<GuardPosition>) {
    let mut map = HashMap::new();
    for pos in route {
        map.entry(pos.coordinates)
            .or_insert(vec![])
            .push(pos.direction);
    }

    for x in 0..lab_map.len() {
        for y in 0..lab_map[0].len() {
            match map.get(&(x, y)) {
                Some(dir) => match dir.len() {
                    0 => print!("?"),
                    1 => match dir[0] {
                        Direction::Up => print!("^"),
                        Direction::Right => print!(">"),
                        Direction::Down => print!("v"),
                        Direction::Left => print!("<"),
                    },
                    l => print!("{}", l),
                },
                None => print!("."),
            }
        }
        println!();
    }
}

fn calulate_guard_route(pos: &GuardPosition, lab_map: &Vec<Vec<char>>) -> GuardRouteOutcome {
    let (mut i, mut j) = pos.coordinates;
    let mut dir = pos.direction;
    let mut positions: Vec<GuardPosition> = vec![*pos];

    loop {
        match patrol_protocol(&i, &j, &dir, &lab_map) {
            PatrolProtocolOutcome::Move((new_i, new_j)) => {
                i = new_i;
                j = new_j;
                match positions
                    .binary_search_by(|p| p.coordinates.cmp(&(i, j)).then(p.direction.cmp(&dir)))
                {
                    Ok(_) => {
                        // _print_guard_route(lab_map, &positions);
                        return GuardRouteOutcome::LoopDetected;
                    }
                    Err(index) => positions.insert(index, GuardPosition::new((i, j), dir)),
                }
            }
            PatrolProtocolOutcome::Turn(direction) => dir = direction,
            PatrolProtocolOutcome::Exit => return GuardRouteOutcome::Positions(positions),
        }
    }
}

fn sum_visited_guard_positions() {
    let input = fs::read_to_string("src/input/day6.txt").unwrap();
    let lab_map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let (i, j) = find_guard(&lab_map).unwrap();
    let outcome = calulate_guard_route(&GuardPosition::new((i, j), Direction::Up), &lab_map);
    if let GuardRouteOutcome::Positions(positions) = outcome {
        // remove duplicates coordinates to get distinct positions the guard visited (regarless of direction)
        let mut coordinates = positions
            .into_iter()
            .map(|p| p.coordinates)
            .collect::<Vec<(usize, usize)>>();
        coordinates.sort();
        coordinates.dedup();

        println!(
            "The sum of visited guard positions is {}",
            coordinates.len()
        );
    } else {
        panic!()
    }
}

fn _sum_candidate_obstacle_positions() {
    let input = fs::read_to_string("src/input/day6.txt").unwrap();
    let lab_map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let (original_i, original_j) = find_guard(&lab_map).unwrap();
    let mut i = original_i;
    let mut j = original_j;
    let mut dir = Direction::Up;
    let mut obstacles: Vec<(usize, usize)> = vec![];

    loop {
        match patrol_protocol(&i, &j, &dir, &lab_map) {
            PatrolProtocolOutcome::Move((new_i, new_j)) => {
                i = new_i;
                j = new_j;

                // add a candidate obstacle at the current position and look for an infinite loop
                let mut altered_lab_map = lab_map.clone();
                assert!(altered_lab_map[i][j] != '#');
                altered_lab_map[i][j] = '#';
                let outcome = calulate_guard_route(
                    &GuardPosition::new((original_i, original_j), Direction::Up),
                    &altered_lab_map,
                );

                if let GuardRouteOutcome::LoopDetected = outcome {
                    match obstacles.binary_search(&(i, j)) {
                        Ok(_) => (),
                        Err(index) => obstacles.insert(index, (i, j)),
                    }
                }
            }
            PatrolProtocolOutcome::Turn(direction) => dir = direction,
            PatrolProtocolOutcome::Exit => break,
        }
    }

    println!(
        "The sum of candidate obstacle positions is {}",
        obstacles.len()
    );
}

fn sum_bridge_calibrations_from_operations(operator_set: &Vec<(&str, fn(u64, u64) -> u64)>) {
    let input = fs::read_to_string("src/input/day7.txt").unwrap();
    let total: u64 = input
        .lines()
        .filter_map(|l| {
            let (value, operands): (&str, &str) = l.split(':').collect_tuple().unwrap();
            let value = value.parse::<u64>().unwrap();
            let operands: Vec<_> = operands
                .split_whitespace()
                .map(|o| o.parse::<u64>().unwrap())
                .collect();
            for operators in (0..operands.len() - 1)
                .map(|_| operator_set)
                .multi_cartesian_product()
            {
                let result = operands
                    .iter()
                    .skip(1)
                    .enumerate()
                    .fold(operands[0], |lhs, (i, rhs)| operators[i].1(lhs, *rhs));

                if result == value {
                    return Some(result);
                }
            }

            None::<u64>
        })
        .sum();

    println!(
        "The sum of bridge calibrations from operations {:?} is {}",
        operator_set.iter().map(|o| o.0).collect::<Vec<_>>(),
        total
    );
}

fn sum_bridge_calibrations_from_two_operations() {
    sum_bridge_calibrations_from_operations(&vec![("+", |l, r| l + r), ("*", |l, r| l * r)]);
}

fn _sum_bridge_calibrations_from_three_operations() {
    sum_bridge_calibrations_from_operations(&vec![
        ("+", |l, r| l + r),
        ("*", |l, r| l * r),
        ("||", |l, r| {
            (l.to_string() + &r.to_string()).parse().unwrap()
        }),
    ]);
}

fn _sum_unique_antinode_locations() {
    let input = fs::read_to_string("src/input/day8.txt").unwrap();
    let lab_map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: Vec<(usize, usize)> = vec![];

    // Read the map, creating a hashmap of antenna frequencies (keys) and locations (values)
    for i in 0..lab_map.len() {
        for j in 0..lab_map[i].len() {
            if lab_map[i][j].is_ascii_alphanumeric() {
                antenna_locations
                    .entry(lab_map[i][j])
                    .or_insert(vec![])
                    .push((i, j));
            }
        }
    }

    // For each pair of antennae at a given freqency, calculate their two antinodes
    for (_, locations) in antenna_locations.iter() {
        for perm in locations.iter().combinations(2) {
            let diff_x = perm[0].0 as i64 - perm[1].0 as i64;
            let diff_y = perm[0].1 as i64 - perm[1].1 as i64;

            // Save the antinodes if they are within the bounds of the map
            let in_bounds = |x: i64, y: i64| {
                x >= 0 && (x as usize) < lab_map.len() && y >= 0 && (y as usize) < lab_map[0].len()
            };

            let antinode = (perm[0].0 as i64 + diff_x, perm[0].1 as i64 + diff_y);
            if in_bounds(antinode.0, antinode.1) {
                antinodes.push((antinode.0 as usize, antinode.1 as usize));
            }

            let antinode = (perm[1].0 as i64 - diff_x, perm[1].1 as i64 - diff_y);
            if in_bounds(antinode.0, antinode.1) {
                antinodes.push((antinode.0 as usize, antinode.1 as usize));
            }
        }
    }

    println!(
        "The sum of unique antinode locations is {}",
        antinodes.iter().unique().count()
    );
}

fn _sum_unique_antinode_locations_accounting_for_resonant_harmonics() {
    let input = fs::read_to_string("src/input/day8.txt").unwrap();
    let lab_map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: Vec<(usize, usize)> = vec![];

    // Read the map, creating a hashmap of antenna frequencies (keys) and locations (values)
    for i in 0..lab_map.len() {
        for j in 0..lab_map[i].len() {
            if lab_map[i][j].is_ascii_alphanumeric() {
                antenna_locations
                    .entry(lab_map[i][j])
                    .or_insert(vec![])
                    .push((i, j));
            }
        }
    }

    // For each pair of antennae at a given freqency, calculate their antinodes
    for (_, locations) in antenna_locations.iter() {
        for combo in locations.iter().combinations(2) {
            let diff_x = combo[0].0 as i64 - combo[1].0 as i64;
            let diff_y = combo[0].1 as i64 - combo[1].1 as i64;

            // Save the antinodes if they are within the bounds of the map
            let in_bounds = |x: i64, y: i64| {
                x >= 0 && (x as usize) < lab_map.len() && y >= 0 && (y as usize) < lab_map[0].len()
            };

            // Search for antinodes in one direction
            let (mut x, mut y) = (combo[0].0 as i64, combo[0].1 as i64);
            antinodes.push((x as usize, y as usize)); // each antenna is an antinode
            loop {
                let antinode = (x + diff_x, y + diff_y);
                if !in_bounds(antinode.0, antinode.1) {
                    break;
                }

                antinodes.push((antinode.0 as usize, antinode.1 as usize));
                (x, y) = antinode;
            }

            // Search for antinodes in the other direction
            let (mut x, mut y) = (combo[1].0 as i64, combo[1].1 as i64);
            antinodes.push((x as usize, y as usize)); // each antenna is an antinode
            loop {
                let antinode = (x - diff_x, y - diff_y);
                if !in_bounds(antinode.0, antinode.1) {
                    break;
                }

                antinodes.push((antinode.0 as usize, antinode.1 as usize));
                (x, y) = antinode;
            }
        }
    }

    println!(
        "The sum of unique antinode locations is {}",
        antinodes.iter().unique().count()
    );
}

fn compute_filesystem_checksum_following_block_compaction() {
    let diskmap = fs::read_to_string("src/input/day9.txt")
        .unwrap()
        .lines()
        .exactly_one()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect_vec();

    let mut disk_model = Vec::new();
    for (id, chunk) in diskmap
        .clone()
        .into_iter()
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        if let Some((num_file, num_free)) = chunk.collect_tuple() {
            for _ in 0..num_file {
                disk_model.push(id);
            }
            for _ in 0..num_free {
                disk_model.push(usize::MAX);
            }
        } else {
            for _ in 0..*diskmap.last().unwrap() {
                disk_model.push(id);
            }
        }
    }

    let mut compacted_model = disk_model.clone();
    let mut free_i: usize = 0;
    let mut file_i: usize = compacted_model.len() - 1;
    loop {
        let mut file_id = compacted_model[file_i];
        while file_id == usize::MAX {
            file_i -= 1;
            if file_i <= free_i {
                break;
            }
            file_id = compacted_model[file_i];
        }

        while compacted_model[free_i] != usize::MAX {
            free_i += 1;
            if file_i <= free_i {
                break;
            }
        }

        compacted_model[free_i] = file_id;
        compacted_model[file_i] = usize::MAX;

        file_i -= 1;
        free_i += 1;
        if file_i <= free_i {
            break;
        }
    }

    let checksum = compacted_model
        .into_iter()
        .enumerate()
        .filter_map(|(i, id)| if id == usize::MAX { None } else { Some(i * id) })
        .sum::<usize>();
    println!(
        "The filesystem checksum following block compaction is {}",
        checksum
    );
}

fn compute_filesystem_checksum_following_file_compaction() {
    let diskmap = fs::read_to_string("src/input/day9.txt")
        .unwrap()
        .lines()
        .exactly_one()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect_vec();

    let mut disk_model = Vec::new();
    for (id, chunk) in diskmap
        .clone()
        .into_iter()
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        if let Some((num_file, num_free)) = chunk.collect_tuple() {
            disk_model.push((id, num_file));
            disk_model.push((usize::MAX, num_free));
        } else {
            disk_model.push((id, *diskmap.last().unwrap()));
        }
    }

    let mut compacted_model = disk_model.clone();
    let mut file_i: usize = compacted_model.len() - 1;
    loop {
        let mut free_i: usize = 0;
        let mut file = compacted_model[file_i];
        while file.0 == usize::MAX {
            file_i -= 1;
            if file_i <= free_i {
                break;
            }
            file = compacted_model[file_i];
        }

        let mut free = compacted_model[free_i];
        while free.0 != usize::MAX || free.1 < file.1 {
            free_i += 1;
            if file_i <= free_i {
                break;
            }
            free = compacted_model[free_i];
        }

        if free_i < file_i {
            assert!(free.1 >= file.1);

            compacted_model[free_i] = file;
            compacted_model[file_i] = (usize::MAX, file.1);
            if file.1 != free.1 {
                file_i += 1;
                free_i += 1;
                compacted_model.insert(free_i, (usize::MAX, free.1 - file.1));
            }
        }

        if file_i == 0 {
            break;
        }

        file_i -= 1;
    }

    let mut model_i = 0;
    let checksum = compacted_model
        .into_iter()
        .filter_map(|region| {
            if region.0 == usize::MAX {
                model_i += region.1 as usize;
                None
            } else {
                let mut sum = 0;
                for _ in 0..region.1 {
                    sum += model_i * region.0;
                    model_i += 1;
                }
                Some(sum)
            }
        })
        .sum::<usize>();
    println!(
        "The filesystem checksum following file compaction is {}",
        checksum
    );
}

fn trail_step(
    i: usize,
    j: usize,
    elevation: usize,
    topo_map: &Vec<Vec<u8>>,
    destinations: &mut Vec<(usize, usize)>,
) -> (usize, usize) {
    if elevation == 9 {
        let result = match destinations.binary_search(&(i, j)) {
            Ok(_) => (0, 1),
            Err(index) => {
                destinations.insert(index, (i, j));
                (1, 1)
            }
        };
        return result;
    }

    let (mut num_dest, mut num_unique) = (0, 0);

    if i > 0 && topo_map[i - 1][j] as usize == elevation + 1 {
        let (d, u) = trail_step(i - 1, j, elevation + 1, topo_map, destinations);
        num_dest += d;
        num_unique += u;
    }

    if j + 1 < topo_map[i].len() && topo_map[i][j + 1] as usize == elevation + 1 {
        let (d, u) = trail_step(i, j + 1, elevation + 1, topo_map, destinations);
        num_dest += d;
        num_unique += u;
    }

    if i + 1 < topo_map.len() && topo_map[i + 1][j] as usize == elevation + 1 {
        let (d, u) = trail_step(i + 1, j, elevation + 1, topo_map, destinations);
        num_dest += d;
        num_unique += u;
    }

    if j > 0 && topo_map[i][j - 1] as usize == elevation + 1 {
        let (d, u) = trail_step(i, j - 1, elevation + 1, topo_map, destinations);
        num_dest += d;
        num_unique += u;
    }

    (num_dest, num_unique)
}

fn sum_scores_of_all_trailheads_on_topo_map() {
    let input = fs::read_to_string("src/input/day10.txt").unwrap();
    let mut topo_map: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        topo_map.push(
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect_vec(),
        );
    }

    // make a pass to discover candidate trailheads
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for (i, row) in topo_map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 0 {
                trailheads.push((i, j));
            }
        }
    }

    // investigate each trailhead to see if there's a trail
    let mut trailhead_scores_sum = 0;
    for (i, j) in trailheads {
        let mut destinations = Vec::new();
        let (d, _) = trail_step(i, j, 0, &topo_map, &mut destinations);
        trailhead_scores_sum += d;
    }

    println!("The sum of trailhead scores is {}", trailhead_scores_sum);
}

fn sum_ratings_of_all_trailheads_on_topo_map() {
    let input = fs::read_to_string("src/input/day10.txt").unwrap();
    let mut topo_map: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        topo_map.push(
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect_vec(),
        );
    }

    // make a pass to discover candidate trailheads
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for (i, row) in topo_map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 0 {
                trailheads.push((i, j));
            }
        }
    }

    // investigate each trailhead to see if there's a trail
    let mut trailhead_ratings_sum = 0;
    for (i, j) in trailheads {
        let mut destinations = Vec::new();
        let (_, u) = trail_step(i, j, 0, &topo_map, &mut destinations);
        trailhead_ratings_sum += u;
    }

    println!("The sum of trailhead ratings is {}", trailhead_ratings_sum);
}

fn sum_stones_after_n_blinks(blinks: usize) {
    let input = fs::read_to_string("src/input/day11.txt").unwrap();

    let mut stones: HashMap<u64, i64> = HashMap::new();
    for stone in input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
    {
        stones
            .entry(stone)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for _ in 0..blinks {
        let mut new_stones: HashMap<u64, i64> = HashMap::new();
        for (key, value) in &stones {
            match key {
                0 => {
                    new_stones
                        .entry(1)
                        .and_modify(|count| *count += value)
                        .or_insert(*value);
                    new_stones
                        .entry(0)
                        .and_modify(|count| *count -= value)
                        .or_insert(-value);
                }
                n => {
                    let s = n.to_string();
                    let len = s.len();
                    if len % 2 == 0 {
                        let nums = s.split_at(len / 2);
                        new_stones
                            .entry(nums.0.parse::<u64>().unwrap())
                            .and_modify(|count| *count += value)
                            .or_insert(*value);
                        new_stones
                            .entry(nums.1.parse::<u64>().unwrap())
                            .and_modify(|count| *count += value)
                            .or_insert(*value);
                        new_stones
                            .entry(*n)
                            .and_modify(|count| *count -= value)
                            .or_insert(-value);
                    } else {
                        new_stones
                            .entry(n * 2024)
                            .and_modify(|count| *count += value)
                            .or_insert(*value);
                        new_stones
                            .entry(*n)
                            .and_modify(|count| *count -= value)
                            .or_insert(-value);
                    }
                }
            }
        }

        for (stone, count) in new_stones {
            stones
                .entry(stone)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }

        stones.retain(|_, v| *v > 0);
    }

    println!(
        "Number of stones after blinking {} times is {}",
        blinks,
        stones.values().sum::<i64>()
    );
}

fn sum_stones_after_25_blinks() {
    sum_stones_after_n_blinks(25);
}

fn sum_stones_after_75_blinks() {
    sum_stones_after_n_blinks(75);
}

#[derive(Clone, Debug)]
struct PlotDetails {
    region: Option<usize>,
    neighbors: usize,
}

impl PlotDetails {
    pub fn new() -> PlotDetails {
        PlotDetails {
            region: None,
            neighbors: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Plot {
    position: (usize, usize),
    details: RefCell<PlotDetails>,
}

impl Plot {
    pub fn new(i: usize, j: usize) -> Plot {
        Plot {
            position: (i, j),
            details: RefCell::new(PlotDetails::new()),
        }
    }
}

fn discover_region(
    plot: &Plot,
    plots: &Vec<Plot>,
    region: usize,
    max: (usize, usize),
) -> (usize, usize) {
    assert!(plot.details.borrow().region.is_none());

    let neighbors = vec![
        if plot.position.0 == 0 {
            None
        } else {
            plots
                .iter()
                .find(|p| p.position == (plot.position.0 - 1, plot.position.1))
        },
        if plot.position.1 == max.1 {
            None
        } else {
            plots
                .iter()
                .find(|p| p.position == (plot.position.0, plot.position.1 + 1))
        },
        if plot.position.0 == max.0 {
            None
        } else {
            plots
                .iter()
                .find(|p| p.position == (plot.position.0 + 1, plot.position.1))
        },
        if plot.position.1 == 0 {
            None
        } else {
            plots
                .iter()
                .find(|p| p.position == (plot.position.0, plot.position.1 - 1))
        },
    ]
    .iter()
    .filter_map(|dir| *dir)
    .collect_vec();

    let num_neighbors = neighbors.len();

    plot.details.borrow_mut().region = Some(region);
    plot.details.borrow_mut().neighbors = num_neighbors;

    let mut area = 0;
    let mut perimeter = 0;
    for neighbor in neighbors {
        if neighbor.details.borrow().region.is_none() {
            let (a, p) = discover_region(neighbor, plots, region, max);
            area += a;
            perimeter += p;
        }
    }

    return (area + 1, perimeter + 4 - num_neighbors);
}

fn total_fencing_price_for_all_regions() {
    let input = fs::read_to_string("src/input/day12.txt").unwrap();
    let mut max_i = 0;
    let mut max_j = 0;
    let mut plot_types: HashMap<char, Vec<Plot>> = HashMap::new();
    for (i, row) in input.lines().enumerate() {
        max_i = i;
        for (j, column) in row.chars().enumerate() {
            max_j = j;
            plot_types
                .entry(column)
                .and_modify(|plots| plots.push(Plot::new(i, j)))
                .or_insert(vec![Plot::new(i, j)]);
        }
    }

    let plot_types = plot_types;
    let mut region_id = 0;
    let mut price = 0;
    for (_, plots) in plot_types {
        for plot in &plots {
            if plot.details.borrow().region.is_none() {
                let (area, perimeter) = discover_region(plot, &plots, region_id, (max_i, max_j));
                region_id += 1;
                price += area * perimeter;
            }
        }
    }

    println!("The total price of fencing all regions is {}", price);
}

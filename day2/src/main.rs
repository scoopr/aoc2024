#[allow(dead_code)]
const EXAMPLE_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn parse_report(input: &str) -> impl Iterator<Item = &str> + Clone + '_ {
    input.lines()
}

fn parse_levels(input: &str) -> impl Iterator<Item = i32> + Clone + '_ {
    input
        .split_whitespace()
        .map(|val| val.parse().expect("number"))
}

fn safe_order(mut input: impl Iterator<Item = i32>) -> bool {
    let first = input.next();
    if let Some(value) = first {
        input
            .try_fold(value.signum(), |acc, x| {
                if x.signum() == acc {
                    Some(acc)
                } else {
                    None
                }
            })
            .is_some()
    } else {
        true
    }
}

fn safe_difference(mut input: impl Iterator<Item = i32>) -> bool {
    input.all(|d| d.abs() >= 1 && d.abs() <= 3)
}

fn check_safety(input: &str, problem_dampener: bool) -> bool {
    let input_values = parse_levels(input).collect::<Vec<_>>();
    let differences = input_values
        .iter()
        .zip(input_values.iter().skip(1))
        .map(|(l, r)| r - l)
        .collect::<Vec<_>>();

    let safe =
        safe_difference(differences.iter().cloned()) && safe_order(differences.iter().cloned());

    if !safe && problem_dampener {
        for i in 0..=differences.len() {
            let iter_skipped = input_values
                .iter()
                .cloned()
                .enumerate()
                .filter_map(|(idx, v)| if idx == i { None } else { Some(v) });

            let differences_skipped = iter_skipped
                .clone()
                .zip(iter_skipped.skip(1))
                .map(|(l, r)| r - l);
            let sd = safe_difference(differences_skipped.clone());
            let so = safe_order(differences_skipped);
            let safe = sd && so;
            if safe {
                return safe;
            }
        }
    }

    safe
}

fn main() {
    let report_safety = parse_report(INPUT).map(|line| check_safety(line, false));
    let safe_count: i32 = report_safety.map(|s| s as i32).sum();

    let report_safety2 = parse_report(INPUT).map(|line| check_safety(line, true));
    let safe_count2: i32 = report_safety2.map(|s| s as i32).sum();

    println!("{safe_count} {safe_count2}");
}

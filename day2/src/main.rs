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

fn check_safety(input: &str) -> bool {
    let mut differences = parse_levels(input)
        .zip(parse_levels(input).skip(1))
        .map(|(l, r)| r - l);

    let safe_order = differences.clone().map(|v| v.signum()).sum::<i32>().abs() as usize
        == differences.clone().count();

    let safe_difference = differences.find(|d| d.abs() < 1 || d.abs() > 3).is_none();
    safe_difference && safe_order
}

fn main() {
    let report_safety = parse_report(INPUT).map(|line| check_safety(line));

    let safe_count: i32 = report_safety.map(|s| s as i32).sum();

    // for safety in report_safety {
    //     println!("{safety}");
    // }
    println!("{safe_count}");
}

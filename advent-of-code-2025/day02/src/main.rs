use advent::{parse_default, positive_ints};

fn main() {
    let input: Vec<Vec<i64>> = parse_default(2, positive_ints, ",");
    assert_eq!(solve(&input, part1), 23560874270);
    assert_eq!(solve(&input, part2), 44143124633);
}

fn part1(s: &str) -> bool {
    let half = &s[..s.len() / 2];
    format!("{}{}", half, half) == s
}

fn part2(s: &str) -> bool {
    let length = s.len();
    (1..=length / 2).any(|i| s[..i].repeat(length / i) == s)
}

fn solve<F>(input: &[Vec<i64>], invalid: F) -> i64
where
    F: Fn(&str) -> bool,
{
    input
        .iter()
        .map(|pair| {
            let (left, right) = (pair[0], pair[1]);
            (left..=right)
                .filter(|n| invalid(&n.to_string()))
                .sum::<i64>()
        })
        .sum()
}

use advent::{Atom, atom, parse_default};

fn main() {
    let input: Vec<(char, i32)> = parse_default(1, atom, "\n")
        .iter()
        .map(|a| match a {
            Atom::Str(s) => (s.as_bytes()[0] as char, s[1..].parse().unwrap()),
            _ => panic!("Expected string atom"),
        })
        .collect();

    let part1 = |dial: i32, _clicks: i32| if dial == 0 { 1 } else { 0 };
    let part2 = |_dial: i32, clicks: i32| clicks;

    assert_eq!(solve(&input, part1), 1154);
    assert_eq!(solve(&input, part2), 6819);
}

fn solve<F>(input: &[(char, i32)], part: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let mut count = 0;
    let mut dial = 50;

    for &(dir, n) in input {
        let total = dial + if dir == 'R' { n } else { -n };
        let clicks = total.div_euclid(100);
        dial = total.rem_euclid(100);
        count += part(dial, clicks.abs());
    }

    count
}

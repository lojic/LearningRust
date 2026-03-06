use std::fmt;
use std::fs;

/// A value that is either an int, float, or string.
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i64),
    Float(f64),
    Str(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Int(n) => write!(f, "{}", n),
            Atom::Float(x) => write!(f, "{}", x),
            Atom::Str(s) => write!(f, "{}", s),
        }
    }
}

/// Parse text into a single int, float, or string.
pub fn atom(text: &str) -> Atom {
    if let Ok(x) = text.parse::<f64>() {
        let rounded = x.round() as i64;
        if rounded as f64 == x {
            Atom::Int(rounded)
        } else {
            Atom::Float(x)
        }
    } else {
        Atom::Str(text.to_string())
    }
}

/// Truncate with defaults: left=70, right=25, dots=" ... ".
pub fn truncate(s: &str) -> String {
    trunc(s, 70, 25, " ... ")
}

/// All of string s if it fits; else left and right ends with dots in the middle.
pub fn trunc(s: &str, left: usize, right: usize, dots: &str) -> String {
    let char_count = s.len();

    if char_count <= left + right + dots.len() {
        s.to_string()
    } else {
        let start: String = s.chars().take(left).collect();
        let end: String = s.chars().skip(char_count - right).collect();
        format!("{}{}{}", start, dots, end)
    }
}

/// Parse with defaults: print_lines=None, do_rstrip=true, root=".".
pub fn parse_default<T, F>(day: u32, parser: F, sep: &str) -> Vec<T>
where
    F: Fn(&str) -> T,
    T: fmt::Debug,
{
    parse(day, parser, sep, None, true, ".")
}

/// Split the day's input file into entries separated by `sep`, and apply `parser` to each.
///
/// This is a port of the Python `parse` function from `app/advent.py`.
///
/// # Examples
/// ```
/// use advent::parse;
///
/// // Parse day 1 input, one string per line (default separator)
/// let lines: Vec<String> = parse(1, |s| s.to_string(), "\n", None, true, "tests");
///
/// // Parse with a custom parser and separator
/// let sections: Vec<Vec<i64>> = parse(5, |s| {
///     s.split_whitespace().filter_map(|w| w.parse().ok()).collect()
/// }, "\n\n", None, true, "tests");
/// ```
pub fn parse<T, F>(
    day: u32,
    parser: F,
    sep: &str,
    print_lines: Option<usize>,
    do_rstrip: bool,
    root: &str,
) -> Vec<T>
where
    F: Fn(&str) -> T,
    T: fmt::Debug,
{
    let fname = format!("{}/day{:02}.txt", root, day);
    let mut text =
        fs::read_to_string(&fname).unwrap_or_else(|e| panic!("Could not read {}: {}", fname, e));

    if do_rstrip {
        let trimmed = text.trim_end();
        text.truncate(trimmed.len());
    }

    let entries: Vec<T> = text.split(sep).map(&parser).collect();

    if let Some(n) = print_lines {
        let all_lines: Vec<&str> = text.lines().collect();
        let lines = &all_lines[..n.min(all_lines.len())];
        let dash = "-".repeat(100);

        println!(
            "{}\n{} \u{27A4} {} chars, {} lines; first {} lines:\n{}",
            dash,
            fname,
            text.len(),
            all_lines.len(),
            lines.len(),
            dash
        );

        for line in lines {
            println!("{}", trunc(line, 70, 25, " ... "));
        }

        let entries_str = format!("{:?}", entries);
        println!(
            "{}\nparse({}) \u{27A4} {} entries:\n{}\n{}\n{}",
            dash,
            day,
            entries.len(),
            dash,
            trunc(&entries_str, 70, 25, " ... "),
            dash
        );
    }

    entries
}

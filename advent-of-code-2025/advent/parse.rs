use std::fmt;
use std::fs;

/// All of string s if it fits; else left and right ends with dots in the middle.
fn trunc(s: &str, left: usize, right: usize, dots: &str) -> String {
    let char_count = s.chars().count();
    if char_count <= left + right + dots.len() {
        s.to_string()
    } else {
        let start: String = s.chars().take(left).collect();
        let end: String = s.chars().skip(char_count - right).collect();
        format!("{}{}{}", start, dots, end)
    }
}

/// Split the day's input file into entries separated by `sep`, and apply `parser` to each.
///
/// This is a port of the Python `parse` function from `app/advent.py`.
///
/// # Examples
/// ```
/// // Parse day 1 input, one string per line (default separator)
/// let lines: Vec<String> = parse(1, |s| s.to_string(), "\n", None, "app", true);
///
/// // Parse with a custom parser and separator
/// let sections: Vec<Vec<i64>> = parse(5, |s| {
///     s.split_whitespace().filter_map(|w| w.parse().ok()).collect()
/// }, "\n\n", None, "app", true);
/// ```
fn parse<T, F>(
    day: u32,
    parser: F,
    sep: &str,
    print_lines: Option<usize>,
    root: &str,
    do_rstrip: bool,
) -> Vec<T>
where
    F: Fn(&str) -> T,
    T: fmt::Debug,
{
    let fname = format!("{}/day{:02}.txt", root, day);
    let mut text = fs::read_to_string(&fname)
        .unwrap_or_else(|e| panic!("Could not read {}: {}", fname, e));

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

/// Convenience wrapper with default arguments matching the Python version:
/// `sep="\n"`, `print_lines=None`, `root="app"`, `do_rstrip=true`.
fn parse_default<T, F>(day: u32, parser: F) -> Vec<T>
where
    F: Fn(&str) -> T,
    T: fmt::Debug,
{
    parse(day, parser, "\n", None, "app", true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trunc_short() {
        assert_eq!(trunc("hello", 70, 25, " ... "), "hello");
    }

    #[test]
    fn test_trunc_long() {
        assert_eq!(
            trunc("beginning to the end", 3, 3, " ... "),
            "beg ... end"
        );
    }

    #[test]
    fn test_parse_lines() {
        let lines = parse(1, |s| s.to_string(), "\n", None, "tests", true);
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "95228   20832");
    }

    #[test]
    fn test_parse_sections() {
        let sections = parse(5, |s| s.to_string(), "\n\n", None, "tests", true);
        assert_eq!(sections.len(), 2);
    }
}

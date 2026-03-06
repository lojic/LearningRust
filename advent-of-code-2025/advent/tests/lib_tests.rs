use advent::{parse, positive_ints, trunc};

#[test]
fn test_parse_lines() {
    let lines = parse(1, |s| s.to_string(), "\n", None, true, "tests");
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0], "95228   20832");
}

#[test]
fn test_parse_sections() {
    let sections = parse(5, |s| s.to_string(), "\n\n", None, true, "tests");
    assert_eq!(sections.len(), 2);
}

#[test]
fn test_positive_ints() {
    assert_eq!(positive_ints("11-22, 3247/90"), vec![11, 22, 3247, 90]);
}

#[test]
fn test_trunc_long() {
    assert_eq!(trunc("beginning to the end", 3, 3, " ... "), "beg ... end");
}

#[test]
fn test_trunc_short() {
    assert_eq!(trunc("hello", 70, 25, " ... "), "hello");
}

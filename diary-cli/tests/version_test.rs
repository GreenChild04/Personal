use diary_cli::version::*;

#[test]
#[should_panic(expected="InvalidSeparator")]
fn version_invalid_separator() {
    Version::parse("0.2.4.56").unwrap();
}

#[test]
#[should_panic(expected="InvalidVersion")]
fn version_invalid_char() {
    Version::parse("0.dave.23").unwrap();
}

#[test]
#[should_panic(expected="InvalidVersion")]
fn version_invalid_num() {
    Version::parse("0.1.256").unwrap(); // 255 is the max version num (might change later)
}

#[test]
fn version_compare() {
    let a: Version = Version::new(0, 2, 25);
    let b: Version = Version::new(0, 1, 36);
    assert!(a.is_compatible(b));
}

#[test]
fn version_compare_larger_minor() {
    let a: Version = Version::new(0, 2, 25);
    let b: Version = Version::new(0, 4, 36);
    assert!(!a.is_compatible(b));
}

#[test]
fn version_to_string() {
    let version: Version = Version::new(0, 1, 23);
    assert_eq!(version.to_string(), String::from("0.1.23"));
}
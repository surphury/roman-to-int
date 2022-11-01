#[cfg(test)]
use crate::roman_to_int;

#[test]
fn test_1() {
    let result = roman_to_int("III");
    assert_eq!(result, 3);
}

#[test]
fn test_2() {
    let result = roman_to_int("LVIII");
    assert_eq!(result, 58);
}

#[test]
fn test_3() {
    let result = roman_to_int("MCMXCIV");
    assert_eq!(result, 1994);
}

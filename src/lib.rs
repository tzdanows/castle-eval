//! Pure functions the HTTP layer exposes. Tested here; the handlers stay thin.

use regex::Regex;

/// Lowercase ASCII words joined by single hyphens; everything else dropped.
pub fn slugify(input: &str) -> String {
    let re = Regex::new(r"[^a-z0-9]+").expect("static pattern");
    re.replace_all(&input.to_ascii_lowercase(), "-")
        .trim_matches('-')
        .to_string()
}

/// Sum of a list, saturating rather than overflowing.
pub fn checked_sum(values: &[i64]) -> i64 {
    values.iter().fold(0i64, |acc, v| acc.saturating_add(*v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_collapses_punctuation() {
        assert_eq!(slugify("Hello, World!  Again"), "hello-world-again");
    }

    #[test]
    fn slug_of_empty_is_empty() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn sum_saturates() {
        assert_eq!(checked_sum(&[i64::MAX, 1]), i64::MAX);
    }

    #[test]
    fn sum_of_nothing_is_zero() {
        assert_eq!(checked_sum(&[]), 0);
    }
}

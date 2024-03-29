use std::{cmp, collections::HashSet, usize};

pub trait Stringr {
    fn remove_chars(&self, chars: HashSet<char>) -> String;
    fn remove_whitespace(&self) -> String;
    fn splitn(&self, n: usize) -> Vec<String>;
    fn splitn_separator(&self, n: usize, separator: &str) -> String;
    fn wildcard_match(
        &self,
        pattern: &str,
        wildcard: &char,
        single_wildcard: &char,
        ignore_casing: bool,
    ) -> bool;
}

impl Stringr for String {
    #[inline]
    fn remove_chars(&self, chars: HashSet<char>) -> String {
        crate::remove_chars(self, chars)
    }

    #[inline]
    fn remove_whitespace(&self) -> String {
        crate::remove_whitespace(self)
    }

    #[inline]
    fn splitn(&self, n: usize) -> Vec<String> {
        crate::splitn(self, n)
    }

    #[inline]
    fn splitn_separator(&self, n: usize, separator: &str) -> String {
        crate::splitn_separator(self, n, separator)
    }

    #[inline]
    fn wildcard_match(
        &self,
        pattern: &str,
        wildcard: &char,
        single_wildcard: &char,
        ignore_casing: bool,
    ) -> bool {
        crate::wildcard_match(self, pattern, wildcard, single_wildcard, ignore_casing)
    }
}

impl Stringr for &str {
    #[inline]
    fn remove_chars(&self, chars: HashSet<char>) -> String {
        crate::remove_chars(self, chars)
    }

    #[inline]
    fn remove_whitespace(&self) -> String {
        crate::remove_whitespace(self)
    }

    #[inline]
    fn splitn(&self, n: usize) -> Vec<String> {
        crate::splitn(self, n)
    }

    #[inline]
    fn splitn_separator(&self, n: usize, separator: &str) -> String {
        crate::splitn_separator(self, n, separator)
    }

    #[inline]
    fn wildcard_match(
        &self,
        pattern: &str,
        wildcard: &char,
        single_wildcard: &char,
        ignore_casing: bool,
    ) -> bool {
        crate::wildcard_match(self, pattern, wildcard, single_wildcard, ignore_casing)
    }
}

/// Returns a new `String` where all specified characters are removed
///
/// # Arguments
///
/// * `input` - Input `String` to remove chars from
/// * `chars` - chars to remove
#[inline]
pub fn remove_chars(input: &str, chars: HashSet<char>) -> String {
    input.chars().filter(|c| !chars.contains(c)).collect()
}

/// Returns a new `String` where all whitespace characters are removed
///
/// Uses `char::is_whitespace` to determine whitespace characters
///
/// # Arguments
///
/// * `input` - Input `String` to remove whitespace chars from
#[inline]
pub fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Splits a `String` every `n`th position
///
/// # Arguments
///
/// * `input` - Input `String` to split
/// * `n` - Number of chars every split has
pub fn splitn(input: &str, n: usize) -> Vec<String> {
    if n == 0 {
        return vec![input.to_string()];
    }

    let size = input.len().div_ceil(n);
    let mut rtn = Vec::with_capacity(size);

    let mut i = 0;

    while i < input.len() {
        let true_len = cmp::min(n, input.len() - i);

        rtn.push(input[i..(i + true_len)].to_string());
        i += true_len;
    }

    rtn
}

pub fn splitn_separator(input: &str, n: usize, separator: &str) -> String {
    if n == 0 || separator.is_empty() {
        return input.to_string();
    }

    let extra_size = input.len().div_ceil(n) * separator.len() - separator.len();
    let new_size = input.len() + extra_size;
    let mut rtn = String::with_capacity(new_size);
    input.chars().enumerate().for_each(|(i, c)| {
        rtn.push(c);
        let j = i + 1;
        if j % n == 0 && j < input.len() {
            rtn.push_str(separator);
        }
    });

    rtn
}

pub fn wildcard_match(
    input: &str,
    pattern: &str,
    wildcard: &char,
    single_wildcard: &char,
    ignore_casing: bool,
) -> bool {
    let mut lookup = vec![vec![false; pattern.len() + 1]; input.len() + 1];
    lookup[0][0] = true;

    let pattern_chars = pattern.chars().collect::<Vec<char>>();
    let input_chars = input.chars().collect::<Vec<char>>();

    for j in 1..pattern.len() + 1 {
        if &pattern_chars[j - 1] == wildcard {
            lookup[0][j] = lookup[0][j - 1];
        }
    }

    let mut i = 1;
    while i <= input.len() {
        let mut j = 1;
        while j <= pattern.len() {
            if pattern_chars[j - 1] == *wildcard {
                lookup[i][j] = lookup[i][j - 1] || lookup[i - 1][j];
            } else if &pattern_chars[j - 1] == single_wildcard {
                lookup[i][j] = lookup[i - 1][j - 1];
            } else if ignore_casing {
                if input_chars[i - 1].to_ascii_lowercase()
                    == pattern_chars[j - 1].to_ascii_lowercase()
                {
                    lookup[i][j] = lookup[i - 1][j - 1];
                } else {
                    lookup[i][j] = false;
                }
            } else if input_chars[i - 1] == pattern_chars[j - 1] {
                lookup[i][j] = lookup[i - 1][j - 1];
            } else {
                lookup[i][j] = false;
            }

            j += 1;
        }

        i += 1;
    }

    lookup[input.len()][pattern.len()]
}

pub fn wildcard_match_default(input: &str, pattern: &str) -> bool {
    wildcard_match(input, pattern, &'*', &'?', false)
}

#[cfg(test)]
mod tests {
    use crate::Stringr;

    #[test]
    fn remove_whitespace() {
        assert_eq!("t e s t".remove_whitespace(), "test")
    }

    #[test]
    fn splitn() {
        assert_eq!(crate::splitn("AEFF??00FE", 0), vec!["AEFF??00FE"]);
        assert_eq!(
            crate::splitn("AEFF??00FE", 1),
            vec!["A", "E", "F", "F", "?", "?", "0", "0", "F", "E"]
        );
        assert_eq!(
            crate::splitn("AEFF??00FE", 2),
            vec!["AE", "FF", "??", "00", "FE"]
        );
        assert_eq!(
            crate::splitn("AEFF??00FE", 3),
            vec!["AEF", "F??", "00F", "E"]
        );
    }

    #[test]
    fn splitn_separator() {
        assert_eq!(
            crate::splitn_separator("AEFF??00FE", 2, " "),
            "AE FF ?? 00 FE"
        );
    }

    #[test]
    fn wildcard_match_default() {
        assert!(crate::wildcard_match_default(
            "longteststring",
            "*teststring"
        ));
        assert!(crate::wildcard_match_default("longteststring", "*test*"));
        assert!(crate::wildcard_match_default(
            "longteststring",
            "l?ngt?st?tring"
        ));
        assert!(crate::wildcard_match_default(
            "longteststring",
            "longteststring"
        ));
        assert!(!crate::wildcard_match_default("longteststring", "*else"));
    }
}

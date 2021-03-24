use std::{cmp, collections::HashSet, usize};

use num::integer;

pub fn remove_chars(input: &String, chars: HashSet<char>) -> String {
    input.chars().filter(|c| !chars.contains(c)).collect()
}

pub fn remove_whitespace(input: &String) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn splitn(input: &String, n: usize) -> Vec<String> {
    let size = integer::div_ceil(input.len(), n);
    let mut rtn = Vec::with_capacity(size);

    let mut i: usize = 0;

    while i < input.len() {
        let true_len = cmp::min(n, input.len() - i);

        rtn.push(input[i..(i + true_len)].to_string());
        i += true_len;
    }

    return rtn;
}

pub fn splitn_separator(input: &String, n: usize, separator: &String) -> String {
    if n <= 0 || separator.is_empty() {
        return input.to_string();
    }

    let extra_size = integer::div_ceil(input.len(), n) * separator.len() - separator.len();
    let new_size = input.len() + extra_size;
    let mut rtn: Vec<String> = Vec::with_capacity(new_size);
    for (i, c) in input.chars().enumerate() {
        rtn.push(c.to_string());
        let j = i + 1;
        if j % n == 0 && j < input.len() {
            rtn.push(separator.to_string());
        }
    }

    return rtn.into_iter().collect();
}

pub fn wildcard_match(
    input: &String,
    pattern: &String,
    wildcard: &char,
    single_wildcard: &char,
    ignore_casing: bool,
) -> bool {
    let mut lookup = vec![vec![false; pattern.len() + 1]; input.len() + 1];
    lookup[0][0] = true;

    for j in 1..pattern.len() + 1 {
        if &pattern.chars().nth(j - 1).unwrap() == wildcard {
            lookup[0][j] = lookup[0][j - 1];
        }
    }

    let mut i = 1;
    while i <= input.len() {
        let mut j = 1;
        while j <= pattern.len() {
            if pattern.chars().nth(j - 1).unwrap() == *wildcard {
                lookup[i][j] = lookup[i][j - 1] || lookup[i - 1][j];
            } else if &pattern.chars().nth(j - 1).unwrap() == single_wildcard {
                lookup[i][j] = lookup[i - 1][j - 1];
            } else {
                if ignore_casing {
                    if input.chars().nth(i - 1).unwrap().to_ascii_lowercase()
                        == pattern.chars().nth(j - 1).unwrap().to_ascii_lowercase()
                    {
                        lookup[i][j] = lookup[i - 1][j - 1];
                    } else {
                        lookup[i][j] = false;
                    }
                } else if input.chars().nth(i - 1).unwrap() == pattern.chars().nth(j - 1).unwrap() {
                    lookup[i][j] = lookup[i - 1][j - 1];
                } else {
                    lookup[i][j] = false;
                }
            }

            j += 1;
        }

        i += 1;
    }

    return lookup[input.len()][pattern.len()];
}

pub fn wildcard_match_default(input: &String, pattern: &String) -> bool {
    wildcard_match(input, pattern, &'*', &'?', false)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn remove_whitespace() {
        assert_eq!(crate::remove_whitespace(&"t e s t".to_string()), "test")
    }

    #[test]
    fn splitn_separator() {
        assert_eq!(
            crate::splitn_separator(&"AEFF??00FE".to_string(), 2, &" ".to_string()),
            "AE FF ?? 00 FE"
        );
    }

    #[test]
    fn wildcard_match_default() {
        assert!(crate::wildcard_match_default(
            &"longteststring".to_string(),
            &"*teststring".to_string()
        ));
        assert!(crate::wildcard_match_default(
            &"longteststring".to_string(),
            &"*test*".to_string()
        ));
        assert!(crate::wildcard_match_default(
            &"longteststring".to_string(),
            &"l?ngt?st?tring".to_string()
        ));
        assert!(crate::wildcard_match_default(
            &"longteststring".to_string(),
            &"longteststring".to_string()
        ));
        assert!(!crate::wildcard_match_default(
            &"longteststring".to_string(),
            &"*else".to_string()
        ));
    }
}

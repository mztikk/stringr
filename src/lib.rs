use std::{cmp, collections::HashSet};

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
}

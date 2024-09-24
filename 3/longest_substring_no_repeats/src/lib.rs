use std::collections::HashSet;

// can use either a backtracking method or a moving window method
// I choose a window moving method this time


pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_set: HashSet<char> = HashSet::new();
    let (mut left, mut longest) = (0, 0);

    for right in 0..s.len() {
        while char_set.contains(&s.chars().nth(right).unwrap()) {
            char_set.remove(&s.chars().nth(left).unwrap());
            left += 1;
        }
        char_set.insert(s.chars().nth(right).unwrap());
        longest = longest.max((right - left + 1) as i32);
    }

    longest
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abcabcbb";
        assert_eq!(length_of_longest_substring(s.to_string()), 3)
    }

    #[test]
    fn case2() {
        let s = "bbbbb";
        assert_eq!(length_of_longest_substring(s.to_string()), 1)
    }

    #[test]
    fn case3() {
        let s = "pwwkew";
        assert_eq!(length_of_longest_substring(s.to_string()), 3)
    }
}

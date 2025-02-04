use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let words = s.split(" ");
    let chars = pattern.chars();

    // different length, not allowed
    if words.clone().count() != chars.clone().count() {
        return false;
    }
    let mut set: HashMap<char, &str> = HashMap::new();
    for a in words.zip(chars) {
        let (word, c) = a;

        // if the word exists already, its false
        if set.iter().find(|(k, v)| **v == word && **k != c).is_some() {
            return false;
        }

        // if the character already has another word, its false
        if set.insert(c, word).is_some_and(|x| x != word) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let pattern = "aaa".to_string();
        let s = "aa aa aa aa".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }

    #[test]
    fn case2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }

    #[test]
    fn case3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }

    #[test]
    fn case34() {
        let pattern = "abba".to_string();
        let s = "dog dog dog dog".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }
}

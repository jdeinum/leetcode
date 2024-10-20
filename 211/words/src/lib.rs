use std::collections::HashMap;

type Link = Option<Box<Node>>;

struct Node {
    pub key: char,
    pub next: HashMap<char, Link>,
}

impl Node {
    fn new(key: char) -> Self {
        Self {
            key,
            next: HashMap::new(),
        }
    }
}

struct WordDictionary {
    dict: HashMap<char, Node>,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }

    fn add_word(&self, word: String) {}

    fn search(&self, word: String) -> bool {}
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

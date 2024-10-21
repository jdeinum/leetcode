use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
};

type Link = Box<Node>;

#[derive(Debug)]
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

#[derive(Debug)]
struct WordDictionary {
    dict: HashMap<char, Box<Node>>,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        let p = word.clone();
        let mut path = p.chars();
        if let Some(start) = path.next() {
            let mut n = self.dict.entry(start).or_insert(Box::new(Node::new(start)));
            for c in path {
                let tmp = n.next.entry(c).or_insert(Box::new(Node::new(c)));
                n = tmp;
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let p = word.clone();
        let mut chars = p.chars();
        let mut ch = chars.next();

        for c in chars {
            match node {
                None => return false,
                Some(n) => node = n.next.get(&c),
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut x = WordDictionary::new();
        assert_eq!(x.search("Hello".to_string()), false);
        x.add_word("Hello".to_string());
        println!("{:?}", x);
        assert_eq!(x.search("Hello".to_string()), true);
        assert_eq!(x.search("Hellos".to_string()), false);
    }
}

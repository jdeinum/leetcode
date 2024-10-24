// misread and did not see the '.' component, sad

use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    pub key: char,
    pub next: HashMap<char, Box<Node>>,
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
        let mut path = p.chars();

        // first, we get the entry for the first letter and check if it is in the dict
        match path.next() {
            Some(start) => {
                let mut n = self.dict.get(&start);
                for c in path {
                    if n.is_none() {
                        return false;
                    }
                    n = n.unwrap().next.get(&c)
                }
                n.is_some()
            }
            None => false,
        }
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
        assert_eq!(x.search("Hello".to_string()), true);
        assert_eq!(x.search("Hellos".to_string()), false);
    }
}

use std::collections::HashMap;

pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let mut word1_hash: HashMap<char, i32> = HashMap::new();
    let mut word2_hash: HashMap<char, i32> = HashMap::new();
    word1
        .chars()
        .for_each(|c| *word1_hash.entry(c).or_insert(0) += 1);

    word2
        .chars()
        .for_each(|c| *word2_hash.entry(c).or_insert(0) += 1);

    let mut all_chars: Vec<char> = word1.chars().chain(word2.chars()).collect();
    all_chars.sort();
    all_chars.dedup();

    all_chars
        .iter()
        .map(|&c| {
            let w1 = word1_hash.get(&c).unwrap_or(&0);
            let w2 = word2_hash.get(&c).unwrap_or(&0);
            (w1 - w2).abs() <= 3
        })
        .all(|x| x == true)
}

#[cfg(test)]
mod tests {
    use crate::check_almost_equivalent;

    #[test]
    fn case1() {
        let word1 = "aaaa".to_string();
        let word2 = "bccb".to_string();
        assert_eq!(check_almost_equivalent(word1, word2), false);
    }

    #[test]
    fn case2() {
        let word1 = "abcdeef".to_string();
        let word2 = "abaaacc".to_string();
        assert_eq!(check_almost_equivalent(word1, word2), true);
    }
}

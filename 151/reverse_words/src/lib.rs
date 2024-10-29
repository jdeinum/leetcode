pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = "the sky is blue";
        assert_eq!(
            reverse_words(words.to_string()),
            "blue is the sky".to_string()
        );
    }
}

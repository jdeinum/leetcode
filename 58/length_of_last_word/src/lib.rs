pub fn length_of_last_word(s: String) -> i32 {
    s.split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.len())
        .last()
        .unwrap_or(0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "Hello World".to_string();
        assert_eq!(length_of_last_word(input), 5);
    }
    #[test]
    fn case2() {
        let input = "   fly me   to   the moon  ".to_string();
        assert_eq!(length_of_last_word(input), 4);
    }
    #[test]
    fn case3() {
        let input = "luffy is still joyboy".to_string();
        assert_eq!(length_of_last_word(input), 6);
    }
}

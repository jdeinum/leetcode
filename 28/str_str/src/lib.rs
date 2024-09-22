pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(a) => a as i32,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let big = "sadbutsad".to_string();
        let small = "sad".to_string();
        assert_eq!(0, str_str(big, small));
    }

    #[test]
    fn case2() {
        let big = "leetcode".to_string();
        let small = "not".to_string();
        assert_eq!(-1, str_str(big, small));
    }
}

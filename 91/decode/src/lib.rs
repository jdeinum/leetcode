pub fn num_decodings(s: String) -> i32 {}

pub fn num_decodings_rec(s: &str) -> i32 {
    let mut count = 0;
    if s.len() > 0 && s.get(0) > "0" {
        count += 1;
    }

    if s.len() > 1 && s[0..=1] >= "10" && s[0.1] <= "25" {
        count += 1;
    }

    return count + num_decodings_rec(s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "12".to_string();
        assert_eq!(num_decodings(s), 2)
    }
    #[test]
    fn case2() {
        let s = "226".to_string();
        assert_eq!(num_decodings(s), 3)
    }

    #[test]
    fn case3() {
        let s = "06".to_string();
        assert_eq!(num_decodings(s), 0)
    }
}

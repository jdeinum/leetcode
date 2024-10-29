pub fn check_string(s: String) -> bool {
    let last_a = s
        .chars()
        .enumerate()
        .filter(|(_, ch)| *ch == 'a')
        .map(|(i, _)| i)
        .last();

    let first_b = s
        .chars()
        .enumerate()
        .find(|(_, ch)| *ch == 'b')
        .map(|(i, _)| i);

    match (last_a, first_b) {
        (Some(a), Some(b)) => a < b,
        _ => true,
    }
}

pub fn check_string_simple(s: String) -> bool {
    let mut seen_b = false;
    for x in s.chars() {
        if x == 'a' && seen_b {
            return false;
        } else if x == 'b' {
            seen_b = true;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "aaabbb".to_string();
        assert!(check_string(s))
    }

    #[test]
    fn case2() {
        let s = "abab".to_string();
        assert_eq!(check_string(s), false)
    }

    #[test]
    fn case3() {
        let s = "bbb".to_string();
        assert!(check_string(s))
    }
}

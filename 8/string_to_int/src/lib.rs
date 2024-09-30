pub fn my_atoi(s: String) -> i32 {
    let num = s
        .chars()
        .skip_while(|a| a.is_whitespace())
        .take_while(|b| b.is_numeric() || b.eq(&'-') || b.eq(&'+'))
        .collect::<String>()
        .parse::<i128>()
        .unwrap_or(0);

    return match i32::try_from(num) {
        Ok(x) => x,
        Err(_) => {
            if num > (i32::MAX as i128) {
                i32::MAX
            } else if num < (i32::MIN as i128) {
                i32::MIN
            } else {
                0
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "42".to_string();
        assert_eq!(my_atoi(s), 42)
    }

    #[test]
    fn case2() {
        let s = "-042".to_string();
        assert_eq!(my_atoi(s), -42)
    }

    #[test]
    fn case3() {
        let s = "1337c0d3".to_string();
        assert_eq!(my_atoi(s), 1337)
    }

    #[test]
    fn case5() {
        let s = "words and 987".to_string();
        assert_eq!(my_atoi(s), 0)
    }

    #[test]
    fn case6() {
        let s = "+-12".to_string();
        assert_eq!(my_atoi(s), 0)
    }

    #[test]
    fn case7() {
        let s = "20000000000000000000".to_string();
        assert_eq!(my_atoi(s), 2147483647)
    }
}

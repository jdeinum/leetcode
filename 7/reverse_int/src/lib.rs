pub fn reverse(x: i32) -> i32 {
    let sign = match x < 0 {
        true => -1,
        false => 1,
    };
    let rev: String = x
        .to_string()
        .chars()
        .rev()
        .filter(|x| x.is_numeric())
        .collect();
    sign * str::parse(&rev).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let x = 123;
        assert_eq!(reverse(x), 321)
    }
    #[test]
    fn case2() {
        let x = -123;
        assert_eq!(reverse(x), -321)
    }
    #[test]
    fn case3() {
        let x = 120;
        assert_eq!(reverse(x), 21)
    }
}

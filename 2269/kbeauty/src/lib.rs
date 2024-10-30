pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    let num_as_str = num.to_string();
    let mut ans = 0;

    num_as_str.as_bytes().windows(k as usize).for_each(|w| {
        let s = String::from_utf8(w.to_vec()).unwrap();
        let s = s.parse::<i32>().unwrap();
        if s != 0 && num % s == 0 {
            ans += 1
        }
    });
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let num = 240;
        let k = 2;
        assert_eq!(divisor_substrings(num, k), 2);
    }
}

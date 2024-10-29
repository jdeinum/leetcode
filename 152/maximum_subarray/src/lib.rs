pub fn max_product(nums: Vec<i32>) -> i32 {
    nums.split(|x| *x < 1)
        .map(|arr| match arr.len() {
            0 => 0,
            _ => arr.iter().product(),
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [2, 3, -2, 4].to_vec();
        assert_eq!(max_product(nums), 6)
    }

    #[test]
    fn case2() {
        let nums = [-2, 0, -1].to_vec();
        assert_eq!(max_product(nums), 0)
    }
}

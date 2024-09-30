pub fn max_area_too_slow(height: Vec<i32>) -> i32 {
    let x = height.iter().enumerate();
    x.clone().fold(0, |acc, (index, val)| {
        let max = x
            .clone()
            .skip(index + 1)
            .map(|(i2, v2)| (i2 - index) as i32 * std::cmp::min(val, v2))
            .max()
            .unwrap_or(0);
        std::cmp::max(max, acc)
    })
}

pub fn max_area(height: Vec<i32>) -> i32 {



}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let x = [1, 1].to_vec();
        assert_eq!(max_area(x), 1);
    }

    #[test]
    fn case2() {
        let x = [1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec();
        assert_eq!(max_area(x), 49);
    }
}

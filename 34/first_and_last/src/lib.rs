// without iterators, we could use a binary search where we

pub fn search_range_easy(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let left: i32 = nums
        .iter()
        .enumerate()
        .find(|(_, &x)| x == target)
        .map(|(a, _)| a as i32)
        .unwrap_or(-1);

    let right: i32 = nums
        .iter()
        .enumerate()
        .rev()
        .find(|(_, &x)| x == target)
        .map(|(a, _)| a as i32)
        .unwrap_or(-1);

    vec![left, right]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [5, 7, 7, 8, 8, 10].to_vec();
        assert_eq!(search_range_easy(nums, 8), vec![3, 4])
    }

    #[test]
    fn case2() {
        let nums = [5, 7, 7, 8, 8, 10].to_vec();
        assert_eq!(search_range_easy(nums, 6), vec![-1, -1])
    }
}

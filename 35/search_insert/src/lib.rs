pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums
        .into_iter()
        .enumerate()
        .filter(|(_, v)| *v <= target)
        .map(|(index, v)| (index as i32, v))
        .last()
    {
        Some((index, v)) if v == target => index,
        Some((index, _)) => index + 1,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [1, 3, 5, 6];
        let target = 5;
        assert_eq!(search_insert(nums.to_vec(), target), 2)
    }

    #[test]
    fn case2() {
        let nums = [1, 3, 5, 6];
        let target = 2;
        assert_eq!(search_insert(nums.to_vec(), target), 1)
    }

    #[test]
    fn case3() {
        let nums = [1, 3, 5, 6];
        let target = 7;
        assert_eq!(search_insert(nums.to_vec(), target), 4)
    }
}

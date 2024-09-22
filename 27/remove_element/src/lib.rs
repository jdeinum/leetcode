pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = [0, 1, 2, 2, 3, 0, 4, 2];
        let target = 2;
        let left = remove_element(&mut nums.to_vec(), target);
        assert_eq!(left, 5);
    }
}

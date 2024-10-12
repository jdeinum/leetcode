use std::collections::HashSet;

// would be better to collect in a Vec and then convert into a hash set and then convert back
// probably
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut acc: HashSet<Vec<i32>> = HashSet::new();
    let mut current: Vec<i32> = Vec::new();
    permute_rec(&mut current, &nums, &mut acc, nums.len());
    acc.into_iter().collect()
}

pub fn permute_rec(
    current: &mut Vec<i32>,
    nums: &Vec<i32>,
    acc: &mut HashSet<Vec<i32>>,
    target: usize,
) {
    nums.iter().enumerate().for_each(|(index, x)| {
        current.push(*x);
        if current.len() == target {
            acc.insert(current.clone());
        }
        let mut c = nums.clone();
        c.remove(index);
        permute_rec(current, &c, acc, target);
        current.pop();
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let x = [1, 1, 2].to_vec();
        assert_eq!(
            permute(x),
            [vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],].to_vec()
        )
    }
}

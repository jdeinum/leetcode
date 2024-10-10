// use a binary search
// for each midpoint, 1 of the 2 halves of the array are sorted
// if our target lies in the sorted half, great! binary search that half
// if it isn't, it must be in the unsorted half, and we'll repeat the same process until left and
// right are equal
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    search_rec(
        nums.iter()
            .enumerate()
            .map(|(a, b)| (a as i32, b.clone()))
            .collect(),
        target,
    )
}

pub fn is_in_range(a: i32, b: i32, target: i32) -> bool {
    return target >= a && target <= b;
}

pub fn search_rec(nums: Vec<(i32, i32)>, target: i32) -> i32 {
    let left_i = 0;
    let right_i = nums.len() - 1;
    let mid_i = (right_i - left_i) / 2;
    let left = nums[left_i];
    let mid = nums[mid_i];

    // base case 1 & 2
    if nums.len() <= 2 {
        let x = nums.iter().find(|(_, val)| *val == target);
        println!("x: {:?}", x);
        return nums
            .iter()
            .find(|(_, val)| *val == target)
            .map_or(-1, |(i, _)| *i);
    }

    // base case 2
    if mid.1 == target {
        return mid.0;
    }

    if left.1 < mid.1 && is_in_range(left.1, mid.1, target) {
        search_rec(nums[left_i..=mid_i].to_vec(), target)
    } else {
        search_rec(nums[mid_i..=right_i].to_vec(), target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search(nums, 0), 4)
    }

    #[test]
    fn case2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search(nums, 3), -1)
    }

    #[test]
    fn case3() {
        let nums = vec![1];
        assert_eq!(search(nums, 0), -1)
    }

    #[test]
    fn case4() {
        let nums = vec![1, 3, 5];
        assert_eq!(search(nums, 1), 0)
    }
}

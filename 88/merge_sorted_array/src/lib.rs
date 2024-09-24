pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let n1 = nums1.clone();
    let it1 = n1.iter().take(m as usize);
    let it2 = nums2.iter().take(n as usize);
    let mut combined: Vec<&i32> = it1.chain(it2).collect();
    combined.sort();
    for i in 0..combined.len() {
        nums1[i] = *combined[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums1 = Vec::from([1, 2, 3, 0, 0, 0]);
        let mut nums2 = Vec::from([2, 5, 6]);
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, Vec::from([1, 2, 2, 3, 5, 6]));
    }
}

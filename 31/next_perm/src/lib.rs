// keep popping numbers out of the vec until you reach one that is smaller than the current one
// in that case, sort the popped values and append it to the end, if you end up with an empty array
// then you can just pop them back in reverse order
pub fn next_permutation(v: Vec<i32>) -> Vec<i32> {
    // find the index i where v[i] > v[i+1]
    let index = v
        .iter()
        .rev()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|y| y[0].1 < y[1].1)
        .map(|[(index, val)]| val)
        .last();

    if index.is_none() {
        let mut x = v.clone();
        x.sort();
        return x;
    }

    let (left, right) = v.split_at(**index.unwrap() as usize);
    let new_right: &mut [i32] = right.clone();
    new_right.sort();
    let mut new = left.clone().to_vec();
    new.append(&mut new_right.to_vec());

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(next_permutation([1, 2, 3].to_vec()), [1, 3, 2].to_vec());
    }

    #[test]
    fn case2() {
        assert_eq!(next_permutation([3, 2, 1].to_vec()), [1, 2, 3].to_vec());
    }
}

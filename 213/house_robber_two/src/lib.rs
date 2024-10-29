pub fn get_candidates(current_run: &mut Vec<(usize, i32)>, houses: &Vec<(usize, i32)>) -> Vec<i32> {
}

pub fn rob_rec(
    current_run: &mut Vec<(usize, i32)>,
    current_max: &mut i32,
    houses: &Vec<(usize, i32)>,
) {
    let m = current_run.iter().map(|(a, b)| b).sum();
    if m > *current_max {
        *current_max = m;
    }

    let cand: Vec<i32> = get_candidates(current_run);
    for c in cand {
        current_run.push(c);
        rob_rec(current_run, current_max, houses);
        current_run.pop();
    }
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut c: Vec<i32> = Vec::new();
    let mut max: i32 = 0;
    rob_rec(
        &mut c,
        &mut max,
        &nums
            .iter()
            .enumerate()
            .map(|(a, b)| (a, b.clone()))
            .collect(),
    );
    max
}

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {}
}

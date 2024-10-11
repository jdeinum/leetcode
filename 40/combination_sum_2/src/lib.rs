use std::collections::{HashMap, HashSet};

pub fn make_move(current_solution: &mut Vec<i32>, x: i32) {
    current_solution.push(x)
}

pub fn unmake_move(current_solution: &mut Vec<i32>) {
    let _ = current_solution.pop();
}

pub fn is_solution(current_solution: &Vec<i32>, target: i32) -> bool {
    current_solution.iter().sum::<i32>() == target
}

pub fn is_unique(current_solution: &Vec<i32>, acc: &HashSet<Vec<i32>>) -> bool {
    let mut x = current_solution.clone();
    x.sort();
    !acc.contains(&x)
}

pub fn get_candidates(
    current_solution: &Vec<i32>,
    candidates: &Vec<i32>,
    target: i32,
    total_counts: &HashMap<i32, i32>,
) -> Vec<i32> {
    let current_sum: i32 = current_solution.iter().sum();
    let mut current_counts: HashMap<i32, i32> = HashMap::new();

    // get the current counts
    current_solution
        .iter()
        .for_each(|&x| *current_counts.entry(x).or_insert(0) += 1);

    // find the candidates
    let mut c: Vec<i32> = candidates
        .iter()
        .filter(|&&x| x <= target - current_sum)
        .filter(|&&x| current_counts.get(&x).unwrap_or(&0) < total_counts.get(&x).unwrap_or(&0))
        .map(|x| x.clone())
        .collect();
    c.dedup();
    c
}

pub fn combination_sum_rec(
    current_solution: &mut Vec<i32>,
    acc: &mut HashSet<Vec<i32>>,
    target: i32,
    nums: &Vec<i32>,
    total_counts: &HashMap<i32, i32>,
) {
    // check if the current solution is a solution
    if is_solution(current_solution, target) && is_unique(current_solution, acc) {
        let mut x = current_solution.clone();
        x.sort();
        acc.insert(x);
    }

    // get the candidates for the next position
    let candidates = get_candidates(current_solution, &nums, target, total_counts);
    for c in candidates {
        make_move(current_solution, c);
        combination_sum_rec(current_solution, acc, target, nums, total_counts);
        unmake_move(current_solution);
    }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut acc: HashSet<Vec<i32>> = HashSet::default();
    let mut current_solution: Vec<i32> = Vec::new();
    let mut total_counts: HashMap<i32, i32> = HashMap::new();
    let mut sorted = candidates.clone();
    sorted.sort();
    candidates
        .iter()
        .for_each(|&x| *total_counts.entry(x).or_insert(0) += 1);
    combination_sum_rec(
        &mut current_solution,
        &mut acc,
        target,
        &sorted,
        &total_counts,
    );
    acc.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let x = [10, 1, 2, 7, 6, 1, 5].to_vec();
        assert_eq!(
            combination_sum(x, 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        )
    }

    #[test]
    fn case2() {
        let x = [2, 5, 2, 1, 2].to_vec();
        assert_eq!(combination_sum(x, 5), vec![vec![1, 2, 2], vec![5]])
    }

    #[test]
    fn case3() {
        let x = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ]
        .to_vec();
        assert_eq!(
            combination_sum(x, 27),
            vec![vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]]
        )
    }
}

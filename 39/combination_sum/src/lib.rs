use std::collections::HashSet;

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

pub fn get_candidates(current_solution: &Vec<i32>, candidates: &Vec<i32>, target: i32) -> Vec<i32> {
    let current_sum: i32 = current_solution.iter().sum();
    candidates
        .iter()
        .filter(|&&x| x <= target - current_sum)
        .map(|y| y.clone())
        .collect::<Vec<i32>>()
}

pub fn combination_sum_rec(
    current_solution: &mut Vec<i32>,
    acc: &mut HashSet<Vec<i32>>,
    target: i32,
    nums: &Vec<i32>,
) {
    // check if the current solution is a solution
    if is_solution(current_solution, target) && is_unique(current_solution, acc) {
        let mut x = current_solution.clone();
        x.sort();
        acc.insert(x);
    }

    // get the candidates for the next position
    let candidates = get_candidates(current_solution, &nums, target);

    // println!(
    //     "target: {} | current_sol: {:?} | candidates: {:?}",
    //     target, current_solution, candidates
    // );

    for c in candidates {
        make_move(current_solution, c);
        combination_sum_rec(current_solution, acc, target, nums);
        unmake_move(current_solution);
    }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut acc: HashSet<Vec<i32>> = HashSet::default();
    let mut current_solution: Vec<i32> = Vec::new();
    combination_sum_rec(&mut current_solution, &mut acc, target, &candidates);
    acc.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let x = [2, 3, 6, 7].to_vec();
        assert_eq!(combination_sum(x, 7), vec![vec![2, 2, 3], vec![7]])
    }

    #[test]
    fn case2() {
        let x = [2, 3, 5].to_vec();
        assert_eq!(
            combination_sum(x, 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        )
    }
}

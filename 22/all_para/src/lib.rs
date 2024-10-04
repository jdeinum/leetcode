pub fn generate_valid_para(n: i32) -> Vec<String> {
    let mut cs: Vec<char> = Vec::default();
    let mut acc: Vec<String> = Vec::default();
    backtrack(&mut cs, &mut acc, n);
    acc
}

pub fn backtrack(current_sol: &mut Vec<char>, acc: &mut Vec<String>, target: i32) {
    // do we have a solution of the right size?
    if current_sol.len() as i32 == 2 * target {
        acc.push(current_sol.iter().collect());
        return;
    }

    // determine all of the potential candidates for the len() position
    let pot_candidates = get_potential_candidates(current_sol, target);

    println!(
        "pot candidates for {:?} are: {:?}",
        current_sol, pot_candidates
    );

    // for each of these potential candidates, recurse
    for c in pot_candidates {
        // make the move
        current_sol.push(c);

        // backtrack again
        backtrack(current_sol, acc, target);

        // unmake the move
        current_sol.pop();
    }
}

pub fn get_potential_candidates(current_sol: &Vec<char>, n: i32) -> Vec<char> {
    let opening = current_sol.iter().filter(|x| **x == '(').count();
    let closed = current_sol.iter().filter(|x| **x == ')').count();

    println!("num opening: {}", opening);
    println!("num closed: {}", closed);

    // no solution yet
    if current_sol.len() == 0 {
        return ['('].to_vec();
    }

    // if the number of opening brackets is equal to n, all we can do is close them off
    if opening as i32 == n {
        return [')'].to_vec();
    }

    // if we are tied, we have to start with an opening bracket
    if opening - closed == 0 {
        return ['('].to_vec();
    }

    // in all other cases, we can do either
    return ['(', ')'].to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(generate_valid_para(1), ["()"].to_vec())
    }

    #[test]
    fn case2() {
        assert_eq!(generate_valid_para(2), ["(())", "()()"].to_vec())
    }

    #[test]
    fn case3() {
        assert_eq!(
            generate_valid_para(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()"].to_vec()
        )
    }
}

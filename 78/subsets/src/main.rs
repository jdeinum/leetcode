use std::collections::HashSet;

type SolutionSet<'a> = &'a mut HashSet<Vec<i32>>;

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: HashSet<Vec<i32>> = HashSet::new();
    res.insert(vec![]);
    let _ = subsets_rec(&mut nums.clone(), &mut res);

    res.iter()
        .map(|x| x.iter().map(|y| y.clone()).collect::<Vec<i32>>())
        .collect()
}

//
// The main idea here is that if we have a current solution set of
// { {}, {a} }
//
// and our next element is b, we'll make a copy of each existing set, add b to it, and take the
// union of the current solution set and these new sets
//
// { {}, {a}, {b}, {ab} }
//
// and we continue doing this until there are no more values left
pub fn subsets_rec(nums: &mut Vec<i32>, acc: SolutionSet) {
    // pop the next element off the list
    let val = match nums.pop() {
        None => return,
        Some(a) => a,
    };

    // Create a new HashSet to store the modified elements
    let mut modified_subsets = HashSet::new();

    // Iterate over the current subsets in acc
    for subset in acc.iter() {
        let mut new_subset = subset.clone();
        new_subset.push(val);
        modified_subsets.insert(new_subset);
    }

    // our union replacement
    for e in modified_subsets {
        if !acc.contains(&e) {
            acc.insert(e);
        }
    }

    let _ = subsets_rec(nums, acc);
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_empty_subset() -> Result<()> {
        let i: Vec<i32> = Vec::new();
        let expected: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(subsets(i), expected);
        Ok(())
    }

    #[test]
    fn test_simple() -> Result<()> {
        let i: Vec<i32> = vec![1, 2];
        let expected: Vec<Vec<i32>> = vec![vec![], vec![1], vec![2], vec![2, 1]];
        let mut res = subsets(i);
        res.sort();
        assert_eq!(res, expected);
        Ok(())
    }
}

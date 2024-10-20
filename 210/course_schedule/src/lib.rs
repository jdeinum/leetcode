// need a way to check for cycles, which DFS can do through back edges

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    top_sort(prerequisites, num_courses)
}

pub fn create_adj_lists(graph: &Vec<Vec<i32>>, num_courses: i32) -> Vec<Vec<i32>> {
    let mut a: Vec<Vec<i32>> = (0..num_courses).map(|_| Vec::new()).collect();
    for entry in graph {
        let course = entry[1];
        let pre = entry[0];
        a[course as usize].push(pre);
    }
    a
}

pub fn top_sort(prerequisites: Vec<Vec<i32>>, num_courses: i32) -> Vec<i32> {
    let mut sol: Vec<i32> = Vec::new();
    let mut discovered: Vec<bool> = (0..num_courses).map(|_| false).collect();
    let al = create_adj_lists(&prerequisites, num_courses);
    for i in 0..num_courses {
        if !discovered[i as usize] {
            dfs(&al, i as i32, &mut discovered, &mut sol);
        }
    }
    sol.into_iter().rev().collect()
}

pub fn dfs(graph: &Vec<Vec<i32>>, node: i32, discovered: &mut Vec<bool>, sol: &mut Vec<i32>) {
    if !discovered[node as usize] {
        discovered[node as usize] = true;
    }

    for prereq in &graph[node as usize] {
        if !discovered[*prereq as usize] {
            dfs(&graph, *prereq, discovered, sol);
        }
    }

    sol.push(node);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let prerequisites = [vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]].to_vec();
        assert_eq!(find_order(4, prerequisites), [0, 2, 1, 3].to_vec())
    }
}

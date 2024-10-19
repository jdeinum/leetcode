pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut v: Vec<i32> = Vec::with_capacity(m as usize * n as usize);
    for _ in 0..m * n {
        v.push(0);
    }

    // for the first row, there is only 1 way to reach each square
    for i in 0..n {
        v[i as usize] = 1;
    }

    // for the first column, there is only 1 way to reach each square
    for i in (0..m * n).step_by(n as usize) {
        v[i as usize] = 1;
    }

    for row in 1..m {
        for column in 1..n {
            let current_cell = row * n + column;
            let previous_cell = row * n + column - 1;
            let previous_row_cell = (row - 1) * n + column;
            v[current_cell as usize] = v[previous_cell as usize] + v[previous_row_cell as usize];
        }
    }

    for row in 0..m {
        for column in 0..n {
            print!("{:2} ", v[(row * n + column) as usize]);
        }
        println!("\n");
    }

    v[(n * m - 1) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(unique_paths(3, 7), 28)
    }

    #[test]
    fn case2() {
        assert_eq!(unique_paths(3, 2), 3)
    }
}

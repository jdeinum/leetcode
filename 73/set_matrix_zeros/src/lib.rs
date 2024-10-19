pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let nrows = matrix.len();
    let ncolumns = matrix[0].len();

    let zeros: Vec<(usize, usize)> = matrix
        .iter()
        .cloned()
        .flatten()
        .enumerate()
        .filter(|(_, v)| *v == 0)
        .map(|(i, _)| (i / ncolumns, i % ncolumns))
        .collect();

    let rows: Vec<usize> = zeros.iter().cloned().map(|(a, _)| a).collect();
    let columns: Vec<usize> = zeros.iter().cloned().map(|(_, b)| b).collect();
    rows.dedup();
    columns.dedup();

    for r in rows {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }
}

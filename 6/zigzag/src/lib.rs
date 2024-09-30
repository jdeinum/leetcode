pub fn convert(s: String, num_rows: i32) -> String {
    let mut zigzags: Vec<(i32, char)> = (0..num_rows)
        .chain((1..num_rows - 1).rev())
        .cycle()
        .zip(s.chars())
        .collect();

    println!("{:?}", zigzags);
    zigzags.sort_by_key(|&(r, _)| r);
    zigzags.into_iter().map(|(_, c)| c).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "PAYPALISHIRING".to_string();
        assert_eq!(convert(s, 3), "PAHNAPLSIIGYIR".to_string())
    }

    #[test]
    fn case2() {
        let s = "PAYPALISHIRING".to_string();
        assert_eq!(convert(s, 4), "PINALSIGYAHRPI".to_string())
    }
}

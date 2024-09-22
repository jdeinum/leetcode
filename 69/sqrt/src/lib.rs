pub fn binary_sqrt(mid_point: i32, x: i32) -> i32 {
    return match mid_point {
        a if a * a == x => a,
        a if (a + 1) * (a + 1) > x && a * a < x => a,
        a if a * a > x => binary_sqrt(mid_point / 2, x),
        _ => binary_sqrt(mid_point + ((x - mid_point) / 2), x),
    };
}

pub fn my_sqrt(x: i32) -> i32 {
    return binary_sqrt(x / 2, x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(my_sqrt(4), 2)
    }

    // left = 0, right = 8, mid = 4
    // left = 0, right = 4, mid = 2
    // left = 0, right = 2, mid = 1
    // left = 2, right = 8, mid =
    #[test]
    fn case2() {
        assert_eq!(my_sqrt(8), 2)
    }
}

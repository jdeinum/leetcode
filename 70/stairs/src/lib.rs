pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut s: Vec<i32> = vec![0; n as usize + 1]; // Initialize the vector with n + 1 elements
    s[1] = 1;
    s[2] = 2;
    for i in 3..=n {
        s[i as usize] = s[i as usize - 2] + s[i as usize - 1];
    }

    println!("{:?}", &s[..]);
    return s[n as usize];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(climb_stairs(2), 2)
    }

    #[test]
    fn case2() {
        assert_eq!(climb_stairs(3), 3)
    }
}

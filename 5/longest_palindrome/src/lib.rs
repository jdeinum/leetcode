pub fn longest_palindrome(s: String) -> String {
    let mut window_size = s.len();
    while window_size > 0 {
        match s
            .as_bytes()
            .windows(window_size)
            .find(|x| x.iter().clone().rev().eq(x.iter()))
        {
            Some(y) => return String::from_utf8(y.to_vec()).unwrap(),
            None => window_size -= 1,
        };
    }

    return String::default();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

pub fn reverse_string(s: &mut Vec<char>) {
    let mut start = 0;
    let mut end = s.len() - 1;
    while end > start {
        s.swap(start, end);
        start += 1;
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::reverse_string;

    #[test]
    fn case1() {
        let mut s = ['h', 'e', 'l', 'l', 'o'].to_vec();
        reverse_string(&mut s);
        assert_eq!(s, ['o', 'l', 'l', 'e', 'h'].to_vec());
    }
}

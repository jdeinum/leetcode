fn main() {}

fn is_paldindrome(x: i32) -> bool {
    let y = x.to_string();
    return y.chars().eq(y.chars().rev());
}

#[cfg(test)]
pub mod tests {
    use crate::is_paldindrome;

    #[test]
    fn yes() {
        let num = 1001;
        assert_eq!(is_paldindrome(num), true)
    }

    #[test]
    fn no() {
        let num = -1001;
        assert_eq!(is_paldindrome(num), false)
    }
}

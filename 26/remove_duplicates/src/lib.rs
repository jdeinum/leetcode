pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

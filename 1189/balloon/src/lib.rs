use std::collections::HashMap;

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut counts: HashMap<char, i32> = HashMap::new();
    text.chars()
        .for_each(|c| *counts.entry(c).or_insert(0) += 1);

    "balon"
        .chars()
        .map(|c| {
            let mut count: i32 = counts.get(&c).unwrap_or(&0).clone();
            if c == 'l' || c == 'o' {
                count /= 2
            }
            count.clone()
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let text = "nlaebolko".to_string();
        assert_eq!(max_number_of_balloons(text), 1)
    }
}

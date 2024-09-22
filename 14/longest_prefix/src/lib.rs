fn find_longest_prefix(strs: Vec<String>) -> String {
    let min_length = strs.iter().map(|x| x.len()).min().unwrap_or(0);
    let mut sol = String::default();
    for x in 0..min_length {
        let prefix = &strs[0][0..x + 1];
        match strs
            .iter()
            .filter(|s| s.as_str().starts_with(prefix))
            .count()
            == strs.len()
        {
            true => sol = prefix.to_string(),
            false => break,
        }
    }

    sol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let vars = ["flower", "flow", "flight"];
        let vars = vars.map(|x| x.to_string()).to_vec();

        let longest_prefix = find_longest_prefix(vars);
        assert_eq!(longest_prefix, "fl".to_string())
    }

    #[test]
    fn case2() {
        let vars = ["dog", "racecar", "car"];
        let vars = vars.map(|x| x.to_string()).to_vec();

        let longest_prefix = find_longest_prefix(vars);
        assert_eq!(longest_prefix, "".to_string())
    }

    #[test]
    fn case3() {
        let vars = ["a"];
        let vars = vars.map(|x| x.to_string()).to_vec();

        let longest_prefix = find_longest_prefix(vars);
        assert_eq!(longest_prefix, "a".to_string())
    }
}

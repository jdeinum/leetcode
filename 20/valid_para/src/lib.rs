fn is_valid_para(s: String) -> bool {
    let opening = std::collections::HashSet::from(["{", "[", "("]);

    let mut stack: Vec<String> = Vec::new();
    for c in s.chars() {
        match opening.contains(c.to_string().as_str()) {
            true => {
                let _ = &stack.push(c.to_string());
                continue;
            }
            false => {
                let back = stack.pop();
                if back.is_none() {
                    return false;
                }
                println!("left: {} | right {}", back.clone().unwrap(), c);
                match (back.unwrap().as_str(), c.to_string().as_str()) {
                    ("{", "}") => continue,
                    ("[", "]") => continue,
                    ("(", ")") => continue,
                    (_, _) => return false,
                };
            }
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case2() {
        let a = "()[]{}";
        assert_eq!(is_valid_para(a.to_string()), true);
    }

    #[test]
    fn case3() {
        let a = "(]";
        assert_eq!(is_valid_para(a.to_string()), false);
    }
}

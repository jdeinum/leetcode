pub fn add_binary(a: String, b: String) -> String {
    let a_rev: String = a.clone().chars().rev().collect();
    let b_rev: String = b.clone().chars().rev().collect();
    return add_bin_rec(a_rev, b_rev, false, &mut String::new());
}

pub fn add_bin_rec(a: String, b: String, carry: bool, acc: &mut String) -> String {
    if a.is_empty() && b.is_empty() && !carry {
        let rev: String = acc.chars().rev().collect();
        return rev;
    }

    let mut count = 0;
    if a.get(0..1) == Some("1") {
        count += 1
    }
    if b.get(0..1) == Some("1") {
        count += 1
    }
    if carry {
        count += 1
    }

    let next_a = match a.get(1..2) {
        Some(_) => a[1..].to_string(),
        None => String::default(),
    };

    let next_b = match b.get(1..2) {
        Some(_) => b[1..].to_string(),
        None => String::default(),
    };

    match count {
        0 => {
            acc.push_str("0");
            add_bin_rec(next_a, next_b, false, acc)
        }
        1 => {
            acc.push_str("1");
            add_bin_rec(next_a, next_b, false, acc)
        }
        2 => {
            acc.push_str("0");
            add_bin_rec(next_a, next_b, true, acc)
        }
        3 => {
            acc.push_str("1");
            add_bin_rec(next_a, next_b, true, acc)
        }
        _ => String::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        assert_eq!("10101".to_string(), add_binary(a, b))
    }

    #[test]
    fn case2() {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!("100".to_string(), add_binary(a, b))
    }
}

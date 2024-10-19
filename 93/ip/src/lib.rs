use std::{str::FromStr, thread::current};

pub fn is_valid_state(current: &str) -> bool {
    if current.len() == 0 {
        return true;
    }

    if current.len() > 0 && &current[0..1] == "." {
        return false;
    }

    current
        .split('.')
        .filter(|x| *x != "")
        .map(|x| u16::from_str(x))
        .any(|x| x.is_err() || x.unwrap() > 256)
}

pub fn get_candidates(current: &str, original: &str) -> Vec<char> {
    if !is_valid_state(current) {
        return vec![];
    }

    return match current.len() {
        0 => original.chars().collect(),
        _ => {
            let num_dots = current.chars().filter(|x| *x == '.').count();
            let mut c: Vec<char> = original
                .chars()
                .skip(current.len() - num_dots)
                .chain((0..4 - num_dots).map(|_| '.'))
                .collect();
            c.sort();
            c.dedup();
            c
        }
    };
}

pub fn make_move(current: &mut String, x: char) {
    current.push(x)
}

pub fn unmake_move(current: &mut String) {
    current.pop();
}

// we can cheat a bit and let the rust std lib do the checking
pub fn is_valid_ip_address(s: &str) -> bool {
    match std::net::Ipv4Addr::from_str(&s) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn restore_ip_addresses_rec(current: &mut String, original: &str, acc: &mut Vec<String>) {
    if is_valid_ip_address(&current) {
        acc.push(current.clone())
    }

    let candidates = get_candidates(&current, original);
    for c in candidates {
        make_move(current, c);
        restore_ip_addresses_rec(current, original, acc);
        unmake_move(current)
    }
}

pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut current: String = String::new();
    let mut acc: Vec<String> = Vec::new();
    restore_ip_addresses_rec(&mut current, &s, &mut acc);
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "25525511135".to_string();
        assert_eq!(
            restore_ip_addresses(s),
            ["255.255.11.135", "255.255.111.35"].to_vec()
        )
    }

    #[test]
    fn case2() {}

    #[test]
    fn case3() {}
}

use std::{collections::HashMap, iter::Peekable};

pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("I", 1);
    map.insert("V", 5);
    map.insert("X", 10);
    map.insert("L", 50);
    map.insert("C", 100);
    map.insert("D", 500);
    map.insert("M", 1000);

    let mut index = 0;
    while index < s.len() {
        let c1 = &s[index..index + 1];
        let c2 = s.get(index + 1..index + 2);
        let r1 = map.get(c1).unwrap();

        if c2.is_none() {
            sum += r1;
            break;
        }
        let r2 = map.get(c2.unwrap()).unwrap_or(&0);

        match r1 < r2 {
            true => {
                sum += r2 - r1;
                index += 1
            }
            false => {
                sum += r1;
            }
        }

        index += 1
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case2() {
        let n = "LVIII".to_string();
        let expected = 58;
        assert_eq!(roman_to_int(n), expected);
    }

    #[test]
    fn case3() {
        let n = "MCMXCIV".to_string();
        let expected = 1994;
        assert_eq!(roman_to_int(n), expected);
    }
}

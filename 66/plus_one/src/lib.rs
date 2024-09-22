// if we run into a 9, we add a zero to the acc and call the function on the rest of the array
// otherwise, we just append the remainder of the reversed digits
// then return the reverse of that

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut rev = digits.clone();
    rev.reverse();
    let mut res = plus_one_rec(&mut rev, &mut Vec::new());
    res.reverse();
    res
}

fn plus_one_rec(reversed: &mut Vec<i32>, acc: &mut Vec<i32>) -> Vec<i32> {
    return match reversed.first() {
        Some(9) => {
            acc.push(0);
            plus_one_rec(&mut reversed[1..].to_vec(), acc)
        }
        Some(a) => {
            acc.push(a + 1);
            acc.append(&mut reversed[1..].to_vec());
            acc.to_vec()
        }
        None => {
            acc.push(1);
            acc.to_vec()

        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [4, 3, 2, 1].to_vec();
        assert_eq!(plus_one(nums), [4, 3, 2, 2].to_vec())
    }

    #[test]
    fn case2() {
        let nums = [9].to_vec();
        assert_eq!(plus_one(nums), [1, 0].to_vec())
    }
}

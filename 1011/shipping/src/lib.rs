pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    // our starting capacity
    let max_weight = weights.iter().max().map_or(0, |&w| w);

    // i would like the starting weight to be a base2 number
    let current_cap: f32 = (max_weight as f32).log2();
    let mut current_cap: i32 = 2_i32.pow(current_cap as u32);

    // find the range we are gonna explore
    loop {
        println!("testing intial cap: {}", current_cap);
        if is_valid(&weights, current_cap, days) {
            break;
        }
        current_cap *= 2;
    }

    let mut left = current_cap / 2;
    let mut right = current_cap;
    let mut mid: i32;

    while left < right {
        mid = (right + left) / 2;
        println!("left: {}", left);
        println!("mid: {}", mid);
        println!("right: {}", right);
        if is_valid(&weights, mid, days) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    // we don't know yet if left is actually valid, if it isn't, we need to add 1 to get the
    // correct answer
    if is_valid(&weights, left, days) {
        return left;
    } else {
        return left + 1;
    }
}

pub fn is_valid(weights: &Vec<i32>, cap: i32, days: i32) -> bool {
    let mut current_index = 0;
    for _ in 0..days {
        let mut current_sum = 0;
        let _: Vec<&i32> = weights
            .iter()
            .skip(current_index)
            .take_while(|&&x| {
                if current_sum + x <= cap {
                    current_sum += x;
                    current_index += 1;
                    true
                } else {
                    false
                }
            })
            .collect();
    }

    return current_index == weights.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let weights = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec();
        let days = 5;
        assert_eq!(ship_within_days(weights, days), 15)
    }

    #[test]
    fn case2() {
        let weights = [3, 2, 2, 4, 1, 4].to_vec();
        let days = 3;
        assert_eq!(ship_within_days(weights, days), 6)
    }

    #[test]
    fn case3() {
        let weights = [1, 2, 3, 1, 1].to_vec();
        let days = 4;
        assert_eq!(ship_within_days(weights, days), 3)
    }

    #[test]
    fn case4() {
        let weights = [10, 50, 100, 100, 50, 100, 100, 100].to_vec();
        let days = 5;
        assert_eq!(ship_within_days(weights, days), 160)
    }
}

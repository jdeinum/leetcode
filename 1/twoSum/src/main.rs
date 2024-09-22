use std::collections::HashMap;

fn main() {}

fn calc_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // num -> index
    let mut num_map: HashMap<i32, i32> = HashMap::default();
    let mut res: Vec<i32> = Vec::new();

    for (i, x) in nums.iter().enumerate() {
        num_map.insert(i as i32, *x);

        // look for the difference
        let difference = target - *x;

        // if the difference exists as a value with a different index, return these indexes
        let pot = num_map
            .iter()
            .filter(|(k, v)| **v == difference && **k != i as i32)
            .last();

        match pot {
            Some(x) => {
                res.push(i as i32);
                res.push(*x.0);
                return res;
            }
            None => continue,
        }
    }
    res
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn it_works() {
        let test_vec = Vec::from([1, 2, 3, 4, 5]);
        let target = 8;
        let indices = calc_two_sum(test_vec, target);
        println!("indices: {:?}", indices);
        assert!(indices.len() == 2);
        assert!(indices.contains(&4));
        assert!(indices.contains(&2));
    }
}

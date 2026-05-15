use std::collections::HashMap;

// leetcode#1
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (index, &num) in nums.iter().enumerate() {
        let complement = target - num;
        match map.get(&complement) {
            Some(&prev_index) => return vec![prev_index as i32, index as i32],
            None => {
                map.insert(num, index);
            }
        }
    }

    vec![] // no solution found
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}

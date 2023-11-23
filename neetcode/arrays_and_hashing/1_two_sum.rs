use std::collections::HashMap;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let target = 9;

    println!("{:?}", Solution::two_sum(nums, target));
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                None => {
                    map.insert(num, index);
                }
                Some(previous_index) => return vec![*previous_index as i32, index as i32],
            }
        }

        return vec![];
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn two_sum_test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}

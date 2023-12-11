use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        if arr.len() < 2 {
            return arr[0];
        }

        let mut result = 0;
        let percent = (arr.len() as f32 * 0.25) as usize;
        let mut map = HashMap::new();

        for i in arr {
            *map.entry(i).or_insert(0) += 1;
        }

        for (&key, &count) in &map {
            if count > percent {
                result = key;
            }
        }

        return result;
    }
}

fn main() {
    let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];

    println!("{}", Solution::find_special_integer(arr));
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn find_special_integer_test() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
        assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
    }
}

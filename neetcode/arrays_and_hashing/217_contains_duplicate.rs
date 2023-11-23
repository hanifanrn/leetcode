use std::collections::HashSet;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 3];

    println!("{}", Solution::contains_duplicate(nums));
}

struct Solution {}

impl Solution {
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut occur: HashSet<i32> = HashSet::new();

        for num in nums {
            if !occur.insert(num) {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn contains_duplicate_test1() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn contains_duplicate_test2() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn contains_duplicate_test3() {
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        )
    }
}

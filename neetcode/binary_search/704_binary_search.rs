fn main() {
    let nums: Vec<i32> = vec![-2, -1, 0, 1, 2, 3, 4, 5, 6];
    let target: i32 = -2;

    println!("{:?}", Solution::search(nums, target));
}

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = nums.len() as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;

            match target - nums[mid as usize] {
                1.. => low = mid + 1,
                0 => {
                    return mid;
                }
                _ => high = mid - 1,
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn search_test() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}

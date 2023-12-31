struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut squared_nums: Vec<i32> = Vec::with_capacity(nums.len());

        for i in nums {
            squared_nums.push(i * i);
        }

        squared_nums.sort();

        return squared_nums;
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, -8, 2, 0, 18, 123, -281];

    println!("{:?}", Solution::sorted_squares(nums));
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn sorted_squares_test() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}

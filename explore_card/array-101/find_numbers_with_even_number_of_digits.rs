struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count_even: i32 = 0;

        for i in nums {
            if i.to_string().len() % 2 == 0 {
                count_even += 1;
            }
        }

        return count_even;
    }
}

fn main() {
    let my_vec: Vec<i32> = vec![1, 2, 22, 127];

    println!("{}", Solution::find_numbers(my_vec));
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn find_numbers_test() {
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
        assert_eq!(Solution::find_numbers(vec![]), 0);
    }
}

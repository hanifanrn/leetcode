use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn largest_substring_between_two_equal_characters(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut res = -1;

        for (i, c) in s.chars().enumerate() {
            match map.get(&c) {
                Some(prev_i) => res = res.max((i - prev_i) as i32 - 1),
                None => {
                    map.insert(c, i);
                }
            }
        }

        return res;
    }
}

fn main() {
    let s: String = String::from("Hanifan");

    println!(
        "{}",
        Solution::largest_substring_between_two_equal_characters(s)
    );
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn largest_substring_between_two_equal_characters_test() {
        assert_eq!(
            Solution::largest_substring_between_two_equal_characters("aa".to_string()),
            0
        );
        assert_eq!(
            Solution::largest_substring_between_two_equal_characters("abca".to_string()),
            2
        );
        assert_eq!(
            Solution::largest_substring_between_two_equal_characters("cbzxy".to_string()),
            -1
        );
    }
}

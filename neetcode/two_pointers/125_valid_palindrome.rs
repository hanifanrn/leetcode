fn main() {
    let s = String::from("hanifan");

    println!("{}", Solution::is_palindrome(s));
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let vec_s = s.chars().collect::<Vec<char>>();

        let mut left = 0;
        let mut right = vec_s.len() - 1;

        while left < right {
            if !vec_s[left].is_ascii_alphanumeric() {
                left += 1;
                continue;
            }

            if !vec_s[right].is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }

            if vec_s[left].to_ascii_lowercase() != vec_s[right].to_ascii_lowercase() {
                return false;
            }

            left += 1;
            right -= 1;
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn is_palindrome_test() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
        assert_eq!(Solution::is_palindrome(String::from(" ")), true);
    }
}

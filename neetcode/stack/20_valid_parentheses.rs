fn main() {
    let s: String = String::from("()}");

    println!("{}", Solution::is_valid(s));
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for i in s.chars() {
            match i {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if i != stack.pop() {
                        return false;
                    }
                }
                _ => (),
            }
        }

        return stack.is_empty();
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn is_valid_test() {
        assert_eq!(Solution::is_valid("()"), true);
        assert_eq!(Solution::is_valid("()[]{}"), true);
        assert_eq!(Solution::is_valid("(]"), false);
        assert_eq!(Solution::is_valid("{"), false);
        assert_eq!(Solution::is_valid(" "), false);
        assert_eq!(Solution::is_valid(""), false);
    }
}

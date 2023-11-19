fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");

    println!("{}", Solution::is_anagram(s, t));
}

struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut res = true;

        if s.len() != t.len() {
            res = false;
        } else {
            let letters = String::from("abcdefghijklmnopqrstuvwxyz");
            let s_vec: Vec<char> = s.chars().collect();
            let t_vec: Vec<char> = t.chars().collect();

            for letter in letters.chars() {
                if s_vec.iter().filter(|&n| *n == letter).count()
                    != t_vec.iter().filter(|&n| *n == letter).count()
                {
                    res = false;
                    break;
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn true_test() {
        let s = String::from("hanifan");
        let t = String::from("nafinah");

        assert_eq!(Solution::is_anagram(s, t), true);
    }

    #[test]
    fn false_test() {
        let s = String::from("yow");
        let t = String::from("hai");

        assert_eq!(Solution::is_anagram(s, t), false);
    }
}

fn main() {
    println!("hello leetcode");
}


struct Solution {}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32) -> i32 {
        println!("TODO");
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn garbage_collection_test() {
        assert_eq!(Solution::garbage_collection(vec!["G","P","GP","GG"], vec![2, 4, 3]), 21);
        assert_eq!(Solution::garbage_collection(vec!["MMM","PGM","GP"], vec![3, 10]), 37);
    }
}

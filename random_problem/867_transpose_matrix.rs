struct Solution {}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n_row = matrix.len();
        let n_col = matrix[0].len();
        let mut matrix_transpose: Vec<Vec<i32>> = vec![vec![0; n_row]; n_col];

        for i in 0..n_col {
            for j in 0..n_row {
                matrix_transpose[i][j] = matrix[j][i];
            }
        }

        return matrix_transpose;
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    println!(
        "[[1,2,3], [4,5,6]] transpose is {:?}",
        Solution::transpose(matrix)
    );
}

#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    fn transpose_test() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }
}

/// See: https://leetcode-cn.com/problems/number-of-submatrices-that-sum-to-target/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut ans = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            // 枚举上边界
            let mut sum = vec![0; n];
            for j in i..m {
                // 枚举下边界
                for c in 0..n {
                    sum[c] += matrix[j][c]; // 更新每列的元素和
                }
                ans += Solution::subarray_sum(&sum, target);
            }
        }
        ans
    }

    fn subarray_sum(nums: &[i32], k: i32) -> i32 {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        mp.insert(0, 1);
        let mut count = 0;
        let mut pre = 0;
        for x in nums.iter() {
            pre += x;
            if mp.contains_key(&(pre - k)) {
                count += mp[&(pre - k)];
            }
            *mp.entry(pre).and_modify(|v| *v += 1).or_insert(1);
        }
        count
    }
}

fn main() {
    assert_eq!(
        Solution::num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0),
        4
    );
    assert_eq!(
        Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0),
        5
    );
    assert_eq!(Solution::num_submatrix_sum_target(vec![vec![904]], 0), 0);
}

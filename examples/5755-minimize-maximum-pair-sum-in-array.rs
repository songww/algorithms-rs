struct Solution;
impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut max = 0;
        let n = nums.len();
        for i in 0..n / 2 {
            max = max.max(nums[i] + nums[n - i - 1]);
        }
        max
    }
}

fn main() {
    assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
}

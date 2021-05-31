struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // (last, now)
        nums.into_iter()
            .fold((0, 0), |(last, now), i| {
                // 这是一个动态规划问题
                (now, now.max(last + i))
            })
            .1
    }
}

fn main() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}

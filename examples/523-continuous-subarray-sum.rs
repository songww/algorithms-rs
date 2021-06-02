use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let mut map = HashMap::with_capacity(nums.len() + 1);
        map.insert(0, -1);
        let mut remainder = 0;

        for i in 0..nums.len() {
            // 若 map中存在 当前余数，且 两下标之间，相差距离大于2
            // 那么，表示 两下标之间 的 连续子数组，为 k 的倍数
            remainder = (remainder + nums[i]) % k; // 计算 加上当前数后，跟 k的倍数 相差多少
            let i = i as i32;
            if let Some(&prev) = map.get(&remainder) {
                if (i - prev) >= 2 {
                    return true;
                }
            } else {
                map.insert(remainder, i as i32);
            }
        }
        false
    }
}

fn main() {
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
}

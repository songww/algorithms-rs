struct Solution;

impl Solution {
    /// 若 `n = 2^x` 且 `x` 为自然数（即 `n` 为 `2` 的幂），则一定满足以下条件：
    ///
    /// 1. 恒有 n & (n - 1) == 0，这是因为：
    ///     `n` 二进制最高位为 `1`，其余所有位为 `0`；
    ///     `n - 1` 二进制最高位为 `0`，其余所有位为 `1`；
    /// 2. 一定满足 `n > 0`。
    pub fn is_power_of_two(n: i32) -> bool {
        if n > 0 {
            n.count_ones() == 1
        } else {
            false
        }
    }
}

fn main() {
    // 示例 1：
    //
    // 输入：n = 1
    // 输出：true
    // 解释：20 = 1
    //
    assert_eq!(Solution::is_power_of_two(1), true);

    // 示例 2：
    //
    // 输入：n = 16
    // 输出：true
    // 解释：24 = 16
    //
    assert_eq!(Solution::is_power_of_two(16), true);

    // 示例 3：
    //
    // 输入：n = 3
    // 输出：false
    //
    assert_eq!(Solution::is_power_of_two(3), false);

    // 示例 4：
    //
    // 输入：n = 4
    // 输出：true
    //
    assert_eq!(Solution::is_power_of_two(4), true);

    // 示例 5：
    //
    // 输入：n = 5
    // 输出：false
    assert_eq!(Solution::is_power_of_two(5), false);
}

struct Solution;

impl Solution {
    const MAX: i32 = 3_i32.pow(19);
    pub fn is_power_of_three(n: i32) -> bool {
        // 3 ** 19 = 1162261467
        n > 0 && Solution::MAX % n == 0
    }
}

fn main() {
    assert_eq!(Solution::is_power_of_three(27), true);
    assert_eq!(Solution::is_power_of_three(0), false);
    assert_eq!(Solution::is_power_of_three(9), true);
    assert_eq!(Solution::is_power_of_three(45), false);
}

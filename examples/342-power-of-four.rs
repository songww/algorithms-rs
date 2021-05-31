struct Solution;
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 || n == 2 || n & (n - 1) != 0 {
            return false;
        }
        n % 3 == 1
    }
}

fn main() {
    assert_eq!(Solution::is_power_of_four(16), true);
    assert_eq!(Solution::is_power_of_four(5), false);
    assert_eq!(Solution::is_power_of_four(1), true);
}

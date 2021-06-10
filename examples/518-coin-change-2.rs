struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for coin in coins {
            for j in coin..=amount {
                let j = j as usize;
                dp[j] += dp[j - coin as usize];
            }
        }
        dp[amount as usize]
    }
}

fn main() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
}

struct Solution;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        const MOD: usize = 1_000_000_007;
        let max_val = *deliciousness.iter().max().unwrap() as usize;
        let max_sum = max_val * 2;
        let mut ans = 0;
        let mut map = vec![0usize; max_sum + 1];
        for delicious in deliciousness.iter() {
            let delicious = *delicious as usize;
            let mut sum = 1;
            while sum <= max_sum {
                let idx = if sum < delicious {
                    map.len() - delicious + sum
                } else {
                    sum - delicious
                };
                ans += map[idx];
                if ans > MOD {
                    ans %= MOD;
                }
                sum <<= 1;
            }
            map[delicious] += 1;
        }
        ans as i32
    }
}

fn main() {
    assert_eq!(Solution::count_pairs(vec![1, 3, 5, 7, 9]), 4);
}

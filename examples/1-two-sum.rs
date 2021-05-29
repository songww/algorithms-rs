struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mapping = std::collections::HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            if mapping.contains_key(&(target - num)) {
                return vec![*mapping.get(&(target - num)).unwrap() as i32, idx as i32]
            }
            mapping.insert(num, idx);
        }
        unreachable!()
    }
}

fn main() {
    assert_eq!(
        Solution::two_sum(vec![50000000, 3, 2, 4, 50000000], 100000000i32),
        vec![0, 4]
    );
}

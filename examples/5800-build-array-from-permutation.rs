struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&num| nums[num as usize]).collect()
    }
}

fn main() {
    assert_eq!(
        Solution::build_array(vec![0, 2, 1, 5, 3, 4]),
        vec![0, 1, 2, 4, 5, 3]
    );
}

struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        if nums.len() == 2 {
            return if nums[0] + nums[1] == 1 { 2 } else { 0 };
        }
        let values = vec![-1, 1];
        let mut neg_prefix = vec![100001usize; 32];
        let mut pos_prefix = vec![100001usize; 32];
        let mut sum: isize = 0;
        let mut max = 0;
        for (idx, v) in nums.into_iter().enumerate() {
            sum += values[v as usize];

            let mut prefix: &mut Vec<usize>;
            let sum_abs: usize = sum.abs() as usize;
            if sum >= 0 {
                if sum as usize >= pos_prefix.len() {
                    pos_prefix.resize(pos_prefix.len() * 2, 100001);
                }
                prefix = &mut pos_prefix;
            } else {
                if sum < 0 && sum.abs() as usize >= neg_prefix.len() {
                    neg_prefix.resize(neg_prefix.len() * 2, 100001);
                }
                prefix = &mut neg_prefix;
            }
            if sum_abs == 0 {
                max = idx + 1;
            } else if prefix[sum_abs] == 100001 {
                println!("{} -> {} : {}", sum, prefix[sum_abs], idx);
                prefix[sum_abs] = idx;
            } else {
                println!("{} -> {} / {}", sum, prefix[sum_abs], idx);
                max = max.max(idx - prefix[sum_abs]);
            }
        }
        max as i32
    }
}

fn main() {
    // assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    // assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    assert_eq!(Solution::find_max_length(vec![0, 1, 1, 0, 1, 1, 1, 0]), 4);
}

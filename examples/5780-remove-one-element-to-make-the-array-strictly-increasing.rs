struct Solution;

impl Solution {
    fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut v = Vec::with_capacity(nums.len());
        v.push(nums[0]);
        let mut i = 1;
        while v.len() < 2 && i < nums.len() {
            if v[0] < nums[i] {
                v.push(nums[i]);
            } else {
                v[0] = nums[i];
            }
            i += 1;
        }
        while i < nums.len() {
            let last1 = v.len() - 1;
            let last2 = last1 - 1;
            if v[last2] < nums[i] && v[last1] < nums[i] {
                v.push(nums[i]);
            } else if v[last2] < nums[i] && v[last1] >= nums[i] {
                v[last1] = nums[i];
            }
            i += 1;
        }
        println!("{:?} {:?}", &nums, &v);
        nums.len() - v.len() < 2
    }
}

fn main() {
    assert_eq!(Solution::can_be_increasing(vec![2, 3, 1, 2]), false);
    println!();
    assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), false);
    println!();
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 3]), true);
    println!();
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]), true);
    println!();
    assert_eq!(Solution::can_be_increasing(vec![100, 21, 100]), true);
    println!();
    assert_eq!(Solution::can_be_increasing(vec![100, 21, 3]), false);
    println!();
    assert_eq!(Solution::can_be_increasing(vec![541, 783, 433, 744]), false);
    println!();
    assert_eq!(Solution::can_be_increasing(vec![105, 924, 32, 968]), true);
}

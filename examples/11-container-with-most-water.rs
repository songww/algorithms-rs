struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let m = if height[left] < height[right] {
                let v = height[left] as usize;
                left += 1;
                v
            } else {
                let v = height[right] as usize;
                right -= 1;
                v
            };
            max = max.max(m * (right - left)) as usize;
        }
        max as i32
    }
}

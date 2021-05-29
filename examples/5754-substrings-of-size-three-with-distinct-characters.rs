struct Solution;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }
        let mut count = 0;
        let mut len = 2;
        let chars: Vec<char> = s.chars().collect();
        while len < s.len() {
            if chars[len] != chars[len - 2]
                && chars[len] != chars[len - 1]
                && chars[len - 2] != chars[len - 1]
            {
                count += 1;
            }
            len += 1;
        }
        count
    }
}

fn main() {
    assert_eq!(Solution::count_good_substrings("aababcabc".to_string()), 4);
    assert_eq!(Solution::count_good_substrings("xyzzaz".to_string()), 1);
}

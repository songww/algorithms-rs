struct Solution;

impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        while s.contains(&part) {
            s = s.replacen(&part, "", 1);
        }
        s
    }
}

fn main() {
    assert_eq!(
        &Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
        "ab"
    );
    assert_eq!(
        &Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
        "dab"
    );
}

struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut cnt = vec![0usize; 26];
        let offset = 'a' as u8 as usize;
        let size = words.len();
        for word in words {
            for c in word.chars() {
                cnt[c as u8 as usize - offset] += 1;
            }
        }
        println!("{:?}", cnt);
        cnt.into_iter().find(|&c| c > 0 && c % size != 0).is_none()
    }
}

fn main() {
    assert_eq!(
        Solution::make_equal(vec![
            "abc".to_string(),
            "aabc".to_string(),
            "bc".to_string()
        ]),
        true
    );
    assert_eq!(
        Solution::make_equal(vec!["ab".to_string(), "a".to_string()]),
        false
    );
    assert_eq!(
        Solution::make_equal(vec![
            "caaaaa".to_string(),
            "aaaaaaaaa".to_string(),
            "a".to_string(),
            "bbb".to_string(),
            "bbbbbbbbb".to_string(),
            "bbb".to_string(),
            "cc".to_string(),
            "cccccccccccc".to_string(),
            "ccccccc".to_string(),
            "ccccccc".to_string(),
            "cc".to_string(),
            "cccc".to_string(),
            "c".to_string(),
            "cccccccc".to_string(),
            "c".to_string()
        ]),
        true
    );
}

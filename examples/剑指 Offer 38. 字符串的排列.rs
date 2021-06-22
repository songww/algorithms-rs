pub struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let mut permutation = Permutation { chars, ans: vec![] };

        permutation.dfs(0);
        permutation.ans
    }
}

use std::iter::FromIterator;

struct Permutation {
    chars: Vec<char>,
    ans: Vec<String>,
}

impl Permutation {
    fn dfs(&mut self, x: usize) {
        if x == self.chars.len() - 1 {
            self.ans.push(String::from_iter(self.chars.iter())); // 添加排列方案
            return;
        }
        let mut seen = vec![false; 26];
        for i in x..self.chars.len() {
            let c = (self.chars[i] as u8 - b'a') as usize;
            if seen[c] {
                continue; // 重复，因此剪枝
            }
            seen[c] = true;
            self.chars.swap(i, x); // 交换，将 c[i] 固定在第 x 位
            self.dfs(x + 1); // 开启固定第 x + 1 位字符
            self.chars.swap(i, x); // 恢复交换
        }
    }
}

fn main() {
    let mut ans = Solution::permutation("abc".to_string());
    ans.sort();
    let mut actual = vec![
        "abc".to_string(),
        "acb".to_string(),
        "bac".to_string(),
        "bca".to_string(),
        "cab".to_string(),
        "cba".to_string(),
    ];
    actual.sort();
    assert_eq!(ans, actual);
}

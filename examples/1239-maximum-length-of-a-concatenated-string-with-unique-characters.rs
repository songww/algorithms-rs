struct Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let solution = Solution {};
        solution
            .max_substring(arr.iter().collect::<Vec<_>>().as_slice())
            .len() as i32
    }

    fn overlap(&self, s1: &str, s2: &str) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        for i in 0..s1.len() {
            for j in 0..s2.len() {
                if s1[i] == s2[j] {
                    return true;
                }
            }
        }
        false
    }
    fn self_overlap(&self, s: &str) -> bool {
        let s = s.as_bytes();
        for i in 0..s.len() - 1 {
            for j in i + 1..s.len() {
                if s[i] == s[j] {
                    return true;
                }
            }
        }
        false
    }

    fn max_substring(&self, arr: &[&String]) -> String {
        let mut dp = vec!["".to_string(); arr.len()];
        if self.self_overlap(&arr[0]) {
            dp[0] = "".to_string();
        } else {
            dp[0] = arr[0].to_owned()
        }
        for i in 1..arr.len() {
            if self.self_overlap(&arr[i]) {
                dp[i] = dp[i - 1].clone();
                continue;
            }
            if !self.overlap(&dp[i - 1], &arr[i]) {
                dp[i] = dp[i - 1].clone() + &arr[i];
                continue;
            }
            let no_arr_i: Vec<_> = arr[..i]
                .into_iter()
                .filter_map(|s| {
                    if !self.overlap(s, &arr[i]) {
                        Some(*s)
                    } else {
                        None
                    }
                })
                .collect();
            let max_arr_i = if no_arr_i.is_empty() {
                arr[i].to_owned()
            } else {
                self.max_substring(no_arr_i.as_slice()) + &arr[i]
            };
            dp[i] = if max_arr_i.len() > dp[i - 1].len() {
                max_arr_i
            } else {
                dp[i - 1].clone()
            };
        }
        dp.pop().unwrap()
    }
}

fn main() {
    assert_eq!(
        Solution::max_length(vec!["un".to_string(), "iq".to_string(), "ue".to_string()]),
        4
    );
    assert_eq!(
        Solution::max_length(vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string()
        ]),
        6
    );
    assert_eq!(
        Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string()]),
        26
    );
}

struct Solution;
impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let xch = x.to_string().chars().next().unwrap();
        let mut n = n;
        if n.starts_with('-') {
            for (idx, ch) in n.char_indices().skip(1) {
                println!("- {} {} {}", idx, ch, xch);
                if ch == '-' {
                    break;
                }
                if ch > xch {
                    n.insert(idx, xch);
                    return n;
                }
            }
            n.push(xch);
            return n;
        } else {
            for (idx, ch) in n.chars().enumerate() {
                println!("+ {} {} {}", idx, ch, xch);
                if ch < xch {
                    n.insert(idx, xch);
                    return n;
                }
            }
            n.push(xch);
            return n;
        };
        unreachable!()
    }
}

fn main() {
    assert_eq!(
        Solution::max_value("-13".to_string(), 2),
        "-123".to_string()
    );
    assert_eq!(Solution::max_value("99".to_string(), 9), "999".to_string());
    assert_eq!(
        Solution::max_value("-132".to_string(), 3),
        "-1323".to_string()
    );
    assert_eq!(
        Solution::max_value("28824579515".to_string(), 8),
        "828824579515".to_string()
    );
    assert_eq!(
        Solution::max_value("469975787943862651173569913153377".to_string(), 3),
        "4699757879438632651173569913153377".to_string()
    );
    assert_eq!(
        Solution::max_value("-648468153646".to_string(), 5),
        "-5648468153646".to_string()
    );
}

struct Solution;

#[derive(Debug)]
struct Part {
    from: usize,
    length: usize,
}

impl Part {
    fn new(from: usize, length: usize) -> Self {
        Self { from, length }
    }

    fn of<'b>(&self, s: &'b str) -> &'b str {
        &s[self.from..self.from + self.length]
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest = Part::new(0, 1);
        let chars: Vec<_> = s.chars().collect();
        for i in 0..chars.len() - 1 {
            let left = Solution::palindrome(&chars, i, i);
            let right = Solution::palindrome(&chars, i, i + 1);
            // println!(
            //     "i {} left {:?} right {:?} longest {:?} ",
            //     i, left, right, longest
            // );
            if left.length > right.length && left.length > longest.length {
                longest = left;
            } else if right.length > longest.length {
                longest = right;
            }
        }
        longest.of(&s).to_string()
    }

    fn palindrome(s: &[char], mut left: usize, mut right: usize) -> Part {
        // println!("+left {}/{} right {}/{}", left, &s[left], right, &s[right]);
        while left <= right && right < s.len() && s[left] == s[right] {
            // println!("-left {}/{} right {}/{}", left, &s[left], right, &s[right]);
            left = left.overflowing_sub(1).0;
            right += 1;
        }

        let left = left.overflowing_add(1).0;
        Part::new(left, right.saturating_sub(left))
    }
}

fn main() {
    assert_eq!(
        Solution::longest_palindrome("babad".to_string()),
        "bab".to_string()
    );
    assert_eq!(
        Solution::longest_palindrome("cbbd".to_string()),
        "bb".to_string()
    );
    println!();
    assert_eq!(
        Solution::longest_palindrome("ccd".to_string()),
        "cc".to_string()
    );
    println!();
    assert_eq!(
        Solution::longest_palindrome("bb".to_string()),
        "bb".to_string()
    );
}

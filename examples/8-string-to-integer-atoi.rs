struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        const SPACE: char = ' ';
        const NEG: char = '-';
        const POS: char = '+';
        const ZERO: char = '0';
        const NINE: char = '9';
        let mut sign = None;
        let mut n = 0;
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == SPACE {
                continue;
            }
            if c == NEG {
                sign = Some(-1);
            } else if c == POS {
                sign = Some(1);
            } else if c < ZERO || c > NINE {
                return 0;
            } else {
                n = (c as u8 - ZERO as u8) as i32;
            }
            break;
        }
        println!("sign {:?}", sign);
        // if positive
        let sign = sign.unwrap_or(1);
        while let Some(c) = chars.next() {
            if c < ZERO || c > NINE {
                break;
            }
            let v = (c as u8 - ZERO as u8) as i32 * sign;
            println!("{} {}", n, v);
            n = if let Some(n) = n.saturating_mul(10).checked_add(v) {
                n
            } else {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            };
        }
        n
    }
}

pub fn main() {
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(Solution::my_atoi("2147483648".to_string()), 2147483647);
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    assert_eq!(Solution::my_atoi("-2147483647".to_string()), -2147483647);
}

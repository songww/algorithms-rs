struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        if Solution::is_perfect_square(n) {
            return 1;
        }
        if Solution::is_four(n) {
            return 4;
        }
        for i in 1..n {
            if i * i >= n {
                break;
            }
            if Solution::is_perfect_square(n - i * i) {
                return 2;
            }
        }
        3
    }

    // 判断是否为完全平方数
    fn is_perfect_square(x: i32) -> bool {
        let y = (x as f64).sqrt() as i32;
        y * y == x
    }

    // 判断是否能表示为 4^k*(8m+7)
    fn is_four(mut x: i32) -> bool {
        while x % 4 == 0 {
            x /= 4;
        }
        x % 8 == 7
    }
}

fn main() {
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
}

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 2 {
            return 2;
        }
        let mut ans = 1;
        for i in 0..n {
            let mut map: HashMap<String, i32> = HashMap::new();
            let mut max = 0;
            for j in i + 1..n {
                let x1 = points[i][0];
                let y1 = points[i][1];
                let x2 = points[j][0];
                let y2 = points[j][1];
                let a = x1 - x2;
                let b = y1 - y2;
                let k = Solution::gcd(a, b);
                let key = format!("{}_{}", a / k, b / k);
                let entry = map.entry(key).or_insert(0);
                *entry += 1;
                max = max.max(*entry);
            }
            ans = ans.max(max + 1);
        }
        ans
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }
}

struct Solution;

impl Solution {
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let mut del = vec![false; 100005];
        let mut low = 0;
        let mut high = removable.len();
        while low < high {
            let mid = (low + high + 1) / 2;
            for i in 0..mid {
                del[removable[i] as usize] = true;
            }
            let mut j = 0;
            for i in 0..s.len() {
                if del[i] {
                    continue;
                }
                if j < p.len() && s.get(i..i + 1) == p.get(j..j + 1) {
                    j += 1;
                }
            }
            for i in 0..mid {
                del[removable[i] as usize] = false;
            }
            if j == p.len() {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        low as i32
    }
}

fn main() {
    assert_eq!(
        Solution::maximum_removals(
            "qobftgcueho".to_string(),
            "obue".to_string(),
            vec![5, 3, 0, 6, 4, 9, 10, 7, 2, 8]
        ),
        7
    );
}

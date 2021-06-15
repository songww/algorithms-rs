struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut idx = arr.len() / 2;
        let mut prev = arr.len();

        while idx > 0 && idx < arr.len() - 1 {
            println!(
                "{}/{} -> {} {} {}",
                idx,
                prev,
                arr[idx - 1],
                arr[idx],
                arr[idx + 1]
            );
            if arr[idx] > arr[idx - 1] && arr[idx] > arr[idx + 1] {
                break;
            }
            let err = (if prev > idx { prev - idx } else { idx - prev }) / 2;
            let err = if err == 0 { 1 } else { err };
            if arr[idx] > arr[idx - 1] && arr[idx] < arr[idx + 1] {
                let n = idx;
                idx = idx + err;
                prev = n;
            } else {
                let n = idx;
                idx = idx - err;
                prev = n;
            }
        }
        idx as i32
    }
}

fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![1, 3, 2]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    println!();
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![
            1, 57, 58, 74, 88, 93, 98, 97, 96, 91, 90, 78, 77, 74, 71, 68, 61, 50, 42, 38, 35, 34,
            26, 20, 15, 14, 5, 4, 2
        ]),
        6
    );
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![18, 29, 38, 59, 98, 100, 99, 98, 90]),
        5
    );
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]),
        2
    );
}

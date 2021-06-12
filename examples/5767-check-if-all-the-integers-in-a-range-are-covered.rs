struct Solution;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, mut left: i32, mut right: i32) -> bool {
        // let values: Vec<_> = (left..=right).collect();
        for range in ranges {
            if left >= range[0] && left <= range[1] {
                left = range[1] + 1;
            }
            if right >= range[0] && right <= range[1] {
                right = range[0] - 1;
            }
            println!("{} {} {} {}", range[0], range[1], left, right);
            if left > right {
                return true;
            }
        }
        false
    }
}

fn main() {
    assert_eq!(
        Solution::is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5),
        true
    );
    println!();
    assert_eq!(
        Solution::is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21),
        false
    );
    println!();
    assert_eq!(
        Solution::is_covered(
            vec![
                vec![36, 50],
                vec![14, 28],
                vec![4, 31],
                vec![24, 37],
                vec![13, 36],
                vec![27, 33],
                vec![23, 32],
                vec![23, 27],
                vec![1, 35]
            ],
            35,
            40
        ),
        true
    );
}

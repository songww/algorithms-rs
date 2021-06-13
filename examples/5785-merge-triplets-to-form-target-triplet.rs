pub struct Solution;

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut merged = vec![0, 0, 0];
        for triplet in &triplets {
            if triplet[0] <= target[0] && triplet[1] <= target[1] && triplet[2] <= target[2] {
                merged[0] = triplet[0].max(merged[0]);
                merged[1] = triplet[1].max(merged[1]);
                merged[2] = triplet[2].max(merged[2]);
            }
            println!("{:?} {:?}", triplet, merged);
            if merged[0] == target[0] && merged[1] == target[1] && merged[2] == target[2] {
                return true;
            }
        }
        false
    }
}

fn main() {
    assert_eq!(
        Solution::merge_triplets(vec![vec![3, 4, 5], vec![4, 5, 6]], vec![3, 2, 5]),
        false
    );
}

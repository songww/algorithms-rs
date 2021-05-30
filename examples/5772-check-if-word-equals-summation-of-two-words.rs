struct Solution;
impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let first: u32 = first_word
            .chars()
            .map(Solution::of_num)
            .rev()
            .enumerate()
            .map(|(i, v)| v * 10u32.pow(i as u32))
            .sum();
        let second: u32 = second_word
            .chars()
            .map(Solution::of_num)
            .rev()
            .enumerate()
            .map(|(i, v)| v * 10u32.pow(i as u32))
            .sum();
        let target: u32 = target_word
            .chars()
            .map(Solution::of_num)
            .rev()
            .enumerate()
            .map(|(i, v)| v * 10u32.pow(i as u32))
            .sum();
        first + second == target
    }

    fn of_num(c: char) -> u32 {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            'i' => 8,
            'j' => 9,
            _ => unreachable!(),
        }
    }
}

fn main() {
    assert_eq!(
        Solution::is_sum_equal("acb".to_string(), "cba".to_string(), "cdb".to_string()),
        true
    );
}

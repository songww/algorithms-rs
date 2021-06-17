struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        const values: [i32; 13] = [1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000];
        const reps: [&str; 13] = [
            "I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M",
        ];

        let mut s = String::new();
        for i in (0..13).rev() {
            while num >= values[i] {
                num -= values[i];
                s.push_str(reps[i]);
            }
        }
        s
    }
}

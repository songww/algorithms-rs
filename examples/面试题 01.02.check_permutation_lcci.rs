impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut s1_counter = vec![0; 128];
        let mut s2_counter = vec![0; 128];
        s1.chars().for_each(|ch| s1_counter[ch as u8 as usize] += 1);
        s2.chars().for_each(|ch| s2_counter[ch as u8 as usize] += 1);

        for i in ('A' as u8 as usize)..128 {
            if s1_counter[i] != s2_counter[i] {
                return false;
            }
        }
        true
    }
}

impl Solution {
    pub fn is_unique(astr: String) -> bool {
        let mut seen: Vec<u8> = vec![0; 128];

        astr.chars()
            .find(|ch| {
                if seen.get(*ch as u8 as usize).unwrap() >= &1 {
                    println!("{} + ", ch);
                    true
                } else {
                    println!("{}", ch);
                    seen[*ch as u8 as usize] += 1;
                    false
                }
            })
            .is_none()
    }
}

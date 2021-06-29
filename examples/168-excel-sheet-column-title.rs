struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        const CHARS: [u8; 27] = [
            'Z' as u8, 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8,
            'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8, 'M' as u8, 'N' as u8, 'O' as u8,
            'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8, 'U' as u8, 'V' as u8, 'W' as u8,
            'X' as u8, 'Y' as u8, 'Z' as u8,
        ];
        const REM: [i32; 26] = [
            26_i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25,
        ];
        let mut name = Vec::new();

        println!("    {}", column_number);
        while column_number > 26 {
            let rem = column_number % 26;
            name.push(CHARS[rem as usize]);
            column_number = (column_number - REM[rem as usize]) / 26;
            println!("{}: {}", column_number, REM[rem as usize]);
        }
        name.push(CHARS[column_number as usize]);
        println!("{}: {}", column_number, column_number);

        let mut column = String::with_capacity(name.len());
        for c in name.into_iter().rev() {
            column.push(c as char);
        }
        column
    }
}

fn main() {
    //
}

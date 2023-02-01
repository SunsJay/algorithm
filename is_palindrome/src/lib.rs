impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut y = x;
        let mut reverted_number = 0;
        // 所有负数不可能为回文
        if y >= 0 {
            while y != 0 {
                reverted_number = reverted_number * 10 + (y % 10);
                y = y / 10;
            }

            if reverted_number == x {
                return true;
            }
        }

        false
    }

    pub fn is_palindrome_two(x: i32) -> bool {
        let s = x.to_string();
        let mut reverted_number = String::with_capacity(s.len());
        for (i, c) in s.chars().rev().enumerate() {
            reverted_number.insert(i, c);
        }

        reverted_number == s
    }
}
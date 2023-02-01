impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut y = x;
        let mut reverted_number = 0;
        // 所有负数和十的倍数不可能为回文
        if y > 0 && y / 10 != 0 {
            while y != 0 {
                reverted_number = reverted_number * 10 + (y % 10);
                y = y / 10;
            }

            if reverted_number == x {
                return  true;
            }
        }

        false
    }
}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x;
        let mut reverted_number = 0;
        // 所有负数和十的倍数不可能为回文
        if x >= 0 && x / 10 != 0 {
            while x != 0 {
                reverted_number = reverted_number * 10 + (x % 10);
                x = x / 10;
            }

            if reverted_number == x {
                return  true;
            }
        }

        false
    }
}
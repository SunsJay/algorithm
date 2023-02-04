impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        // if digits.len() == 1 && digits[0] == 0 {
        //     return vec![1];
        // }
        let mut new_digits: Vec<i32> = digits.clone();
        for i in digits.len() - 1..=0 {
            if digits[i] != 9 {
                new_digits[i] = digits[i] + 1;
                return new_digits;
            } else {
                new_digits[i] = 0;
            }
        }

        if new_digits[0] == 0 {
            new_digits.insert(0, 1);
        }

        new_digits
    }
}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut new_digits: Vec<i32> = digits.clone();
        let mut i = digits.len() - 1;
        while i >= 0 {
            if digits[i] != 9 {
                new_digits[i] = digits[i] + 1;
                return new_digits;
            } else {
                new_digits[i] = 0;
            }
            i -= 1;
        }
        if new_digits[0] == 0 {
            new_digits.insert(0, 1);
        }

        new_digits
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        let a = vec![1, 9, 9];
        let b = Solution::plus_one(a);
        println!("{:?}", b);
    }
}
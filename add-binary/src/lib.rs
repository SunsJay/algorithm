impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let total = 1;
        println!("{}", a.parse::<i32>().unwrap());
        format!("{:b}", total)
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_one() {
        use super::*;
        let a = Solution::add_binary("11".to_string(), "1".to_string());
        println!("{}", a);
    }
}
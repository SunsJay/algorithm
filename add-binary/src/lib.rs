impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let ia = i128::from_str_radix(&a, 2).unwrap();
        let ib = i128::from_str_radix(&b, 2).unwrap();
        format!("{:b}", (ia+ib)).to_string()
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
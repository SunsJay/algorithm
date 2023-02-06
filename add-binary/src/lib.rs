impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let total: i32 = a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap();
        format!("{:b}", total)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

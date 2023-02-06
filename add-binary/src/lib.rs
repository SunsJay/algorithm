impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let total: i32 = a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap();
        format!("{:b}", total)
    }
}


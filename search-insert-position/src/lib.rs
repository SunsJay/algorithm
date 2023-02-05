impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for i in 0..nums.len() {
            if target < nums[i + 1] && target >= nums[i] {
                return i as i32
            }
        }
        0
    }
}

struct Solution{}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_search() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);

    }
}
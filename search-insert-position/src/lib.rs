impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target <= nums[0] {
            return 0;
        }
        for i in 1..nums.len() {

            if target <= nums[i] && target > nums[i - 1] {
                return i as i32;
            }
        }
        nums.len() as i32
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }
}
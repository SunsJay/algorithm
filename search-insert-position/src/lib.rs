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
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
        nums.retain(|&x| x != val ) ;
        nums.len()
    }
}


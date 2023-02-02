impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }

    // 双指针法
    pub fn remove_duplicates_two_points(nums: &mut Vec<i32>) -> i32 {
        let mut j = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[j] {
                nums[j + 1] = nums[i];
                j += 1;
            }
        }

        j as i32  + 1
    }


}
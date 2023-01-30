impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
        nums.retain(|&x| x != val);
        nums.len()
    }

    pub fn remove_element_a(nums: &mut Vec<i32>, val: i32) -> usize {
        if nums.len() == 0 {
            return 0;
        }
        let mut slow_point = 0;


        for pos in 0..nums.len() {
            if nums[pos] != val {
                nums[slow_point] = nums[pos];
                slow_point += 1;
            }
        }

        slow_point
    }
}


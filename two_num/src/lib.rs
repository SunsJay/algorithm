use std::collections::HashMap;

impl Solution {
    // 暴力遍历
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        nums
    }

    // 哈希表法
    pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            match map.get(&(target - nums[i])) {
                None => {
                    map.insert(nums[i], i);
                }
                Some(&v) => {
                    return vec![v as i32, i as i32]
                }
            }
        }
        nums
    }
}


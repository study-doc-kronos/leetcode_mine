use std::collections::HashMap;

impl Solution {
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     if nums.len() == 0 {
    //         return vec![]
    //     }
    //     for i in 0..nums.len() {
    //         for j in i+1..nums.len() {
    //             if nums[j] + nums[i] == target {
    //                 return vec![i as i32, j as i32];
    //             }
    //         }
    //     }
    //     vec![]
    // }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![]
        }
        let mut idx = HashMap::new();
        for (j, &x) in nums.iter().enumerate() {
            if let Some(&i) = idx.get(&(target-x)) {
                return vec![i as i32, j as i32];
            }
            idx.insert(x,j);
        }
        vec![]

    }
}

struct Solution;

fn main() {
    let nums = vec![2,7,9,11];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Indices: {:?}", result);
}
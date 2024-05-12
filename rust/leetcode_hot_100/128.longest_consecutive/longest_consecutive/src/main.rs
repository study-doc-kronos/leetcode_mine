use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // _ 表示编译器应自动推断类型
        let m: HashSet<_> = nums.iter().map(|&v| v).collect();
        nums.into_iter().map(|mut v| {
            let mut max_len = 1;
            if !m.contains(&(v-1)) {
                while m.contains(&(v+1)) {
                    max_len+=1;
                    v+=1;
                }
            }
            max_len
        }).max().unwrap_or(0)
    }
}

struct Solution;

fn main() {
    let nums = vec![1, 2, 3, 4, 1,1];
    let result = Solution::longest_consecutive(nums);
    println!("Result: {}", result);
}

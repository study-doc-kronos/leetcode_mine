impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[left] = nums[i];
                if i != left {
                    nums[i] = 0;
                }
                left+=1;
            }
        }
    }
}

struct Solution;


fn main() {
    let mut nums = vec![1,2,2,0,0,11];
    Solution::move_zeroes(&mut nums);
    println!("Result: {:?}", nums);
}

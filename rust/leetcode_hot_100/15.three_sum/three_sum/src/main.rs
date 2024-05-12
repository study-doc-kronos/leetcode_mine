impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = Vec::new();
        let n = nums.len();
        for i in 0..n-2 {
            let x = nums[i];
            if i > 0 && x == nums[i-1] {
                continue;
            }
            if x + nums[i+1] + nums[i+2] > 0 {
                continue;
            }
            if x + nums[n-1] + nums[n-2] < 0 {
                continue;
            }
            let mut left = i+1;
            let mut right = n-1;
            while left < right {
                let s = x + nums[left] + nums[right];
                if s > 0 {
                    right-=1;
                } else if s < 0 {
                    left += 1;
                } else {
                    res.push(vec![x, nums[left], nums[right]]);
                    left +=1;
                    while left < right && nums[left] == nums[left-1] {
                        left +=1;
                    }
                    right -= 1;
                    while left < right && nums[right] == nums[right+1] {
                        right -= 1;
                    }
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    let nums = vec![1, 2, -3, 4, 5];
    let result = Solution::three_sum(nums);
    println!("Result: {:?}", result);
}

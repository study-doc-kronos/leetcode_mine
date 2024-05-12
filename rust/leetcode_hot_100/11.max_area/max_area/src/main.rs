impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut right = height.len()-1;
        while left < right {
            let area = (right-left) as i32 * height[left].min(height[right]);
            res = res.max(area);
            if height[left] < height[right] {
                left+=1;
            }else {
                right-=1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    let height = vec![1,2,3,4,5,6,7,8,9];
    let result = Solution::max_area(height);
    println!("Result: {:?}", result);
}

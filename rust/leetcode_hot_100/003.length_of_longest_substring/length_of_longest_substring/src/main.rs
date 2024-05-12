impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res  = 0;
        let mut left = 0;
        let mut window = [false;128];
        for (right, &c) in s.iter().enumerate() {
            let cc = c as usize;
            while window[cc] {
                window[s[left] as usize] = false;
                left +=1;
            }
            window[cc] = true;
            res = res.max(right-left+1);
        }
        res as i32
    }
}

struct Solution;

fn main() {
    let s = "safsfsa";
    let result = Solution::length_of_longest_substring(s.to_string());

    println!("Result: {:?}", result);
}

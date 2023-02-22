use std::cmp;

struct Solution { }

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left != right {
            let distance: i32 = (right - left) as i32;
            let area = cmp::min(height[right], height[left]) * distance;

            if area > max {
                max = area;
            }

            if height[right] < height[left] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        return max;
    }
}

fn main() {
    let height: Vec<i32> = vec![9, 3, 1, 7, 5, 15];
    let max = Solution::max_area(height);
    println!("{}", max);
}
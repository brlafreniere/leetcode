// https://leetcode.com/problems/median-of-two-sorted-arrays/

struct Solution {
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums3 = [nums1, nums2].concat();
        nums3.sort();

        // index has to be a whole number, so let count be a whole number
        // if there is one element in the vector, then the middle_count will
        // come out to 0 (rounding down), and the index will be 0.
        // so the only number in the vector will be the median.
        let count = nums3.len();
        println!("count: {}", count);
        let middle_count = (count as f64 / 2.0).ceil() as i32;
        println!("middle_count: {}", middle_count);

        let middle_index;

        if middle_count == 0 {
            middle_index = 0;
        } else {
            middle_index = middle_count - 1;
        }

        let median: f64;

        let is_odd = count & 1 == 1;
        if is_odd {
            // if it's odd we can just get the middle index and that's our median.
            median = nums3[middle_index as usize] as f64;
        } else {
            // if there is an even number of elements, we calculate the median
            // by combining the 2 middle elements and dividing them by 2
            let median_a = nums3[middle_index as usize] as f64;
            let median_b = nums3[(middle_index+1) as usize] as f64;
            median = (median_a + median_b) / 2.0 as f64;
        }
        println!("median: {}", median);

        return median;
    }
}

fn main() {
    let solution = Solution {};
    let nums1 = [4];
    let nums2 = [];
    Solution::find_median_sorted_arrays(nums1.to_vec(), nums2.to_vec());
}
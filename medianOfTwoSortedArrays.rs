// https://leetcode.com/problems/median-of-two-sorted-arrays/

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        // combine the two arrays into nums1, and sort it.
        nums1.append(&mut nums2);
        nums1.sort();
        
        // get the length of nums1, this will determine what happens next
        let count = nums1.len();
        
        let is_odd = |x: i32| x & 1 == 1;
        
        println!("is_odd: {}", is_odd);
        
        println!("count: {}", count);
        println!("nums1: {:?}", nums1);
        
        let result = 0.0;
        return result;
    }
}

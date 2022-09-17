// https://leetcode.com/problems/palindrome-number/

struct Solution {}

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    let x_str: String = x.to_string();
    let x_str_len = x_str.chars().count();
    let x_str_bytes = x_str.as_bytes();

    let mut begin = 0;
    let mut end = x_str_len - 1;

    let mid_point: f32 = x_str_len as f32 / 2.0_f32;
    let mid_point_floored: i32 = mid_point.floor() as i32;

    let mut is_palindrome = true;

    // println!("{}", begin);
    // println!("{}", mid_point);

    while begin < mid_point_floored {
      if x_str_bytes[begin as usize] != x_str_bytes[end as usize] {
        is_palindrome = false;
        break;
      }
      begin += 1;
      end -= 1;
    }
    return is_palindrome;
  }
}

// test calls
pub fn main() {
  let mut input_num = 12345;
  println!("{}: {}", input_num, Solution::is_palindrome(input_num));
  input_num = 1221;
  println!("{}: {}", input_num, Solution::is_palindrome(input_num));
  input_num = 12321;
  println!("{}: {}", input_num, Solution::is_palindrome(input_num));
  input_num = 121321;
  println!("{}: {}", input_num, Solution::is_palindrome(input_num));
}
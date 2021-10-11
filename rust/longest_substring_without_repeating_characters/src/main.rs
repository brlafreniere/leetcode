// Problem URL:
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashMap;
use std::collections::HashSet;

struct SlidingWindowSolution;
impl SlidingWindowSolution {
    pub fn find_longest_substr(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut chars_encountered = HashSet::new();
        let mut max_substr_length = 0;

        while right < s.chars().count() {
            let right_char = s.chars().nth(right).unwrap();
            if !chars_encountered.contains(&right_char) {
                chars_encountered.insert(right_char);
                max_substr_length = std::cmp::max(chars_encountered.len() as i32, max_substr_length);
                right += 1;
            } else {
                let left_char = s.chars().nth(left).unwrap();
                chars_encountered.remove(&left_char);
                left += 1;
            }
        }

        println!("result: {}", max_substr_length);

        return max_substr_length;
    }
}

struct SubstrFinder;
impl SubstrFinder {
    pub fn get_longest_substr(strs: Vec<String>) -> String {
        let mut longest = String::from("");
        for s in strs {
            if s.chars().count() > longest.chars().count() {
                longest = s.clone();
            }
        }
        return longest.to_string();
    }

    pub fn find_non_repeating_substrings(input_str: String) -> Vec<String> {
        //println!("Input String: {:?}", input_str);

        let mut substrings = Vec::new();
        let mut substr_buf = String::from("");
        let mut chars_encountered: HashMap<char, bool> = HashMap::new();


        let mut start_pos = 0;

        while start_pos < input_str.chars().count() {
            let mut chars_iter = input_str.chars().enumerate().skip(start_pos);
            while let Some(chr) = chars_iter.next() {
                let c = chr.1;
                let i = chr.0;

                //println!("c: {:?}", c);

                if chars_encountered.contains_key(&c) {
                    substrings.push(substr_buf.clone());
                    substr_buf = String::from("");
                    chars_encountered.clear();
                }

                substr_buf.push(c);
                chars_encountered.insert(c, true);
            }

            substrings.push(substr_buf.clone());
            substr_buf = String::from("");
            chars_encountered.clear();

            start_pos += 1;
        }

        //println!("result: {:?}", substrings);

        return substrings;
    }
}

struct Solution { }
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // let substrings = SubstrFinder::find_non_repeating_substrings(s);
        // let longest_substr = SubstrFinder::get_longest_substr(substrings);
        println!("input string: {}", s);
        SlidingWindowSolution::find_longest_substr(s);

        return 0;
    }
}

fn main() {
    let input_str = String::from("abcabcbb");
    Solution::length_of_longest_substring(input_str);

    let input_str = String::from("bbbbb");
    Solution::length_of_longest_substring(input_str);

    let input_str = String::from("pwwkew");
    Solution::length_of_longest_substring(input_str);

    let input_str = String::from("");
    Solution::length_of_longest_substring(input_str);

    let input_str = String::from(" ");
    Solution::length_of_longest_substring(input_str);

    let input_str = String::from("aab");
    Solution::length_of_longest_substring(input_str);

    let input_str = String::from("bdb");
    Solution::length_of_longest_substring(input_str);

    let input_str = String::from("dvdf");
    Solution::length_of_longest_substring(input_str);
}
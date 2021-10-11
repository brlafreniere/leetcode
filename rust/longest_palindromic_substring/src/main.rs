struct PalindromeIterator {
    left_index: usize,
    right_index: usize,
    input_string: String
}
impl PalindromeIterator {
    fn new(s: String) -> PalindromeIterator {
        PalindromeIterator {
            left_index: 0,
            right_index: 0,
            input_string: s
        }
    }

    fn increment(&mut self) {
        if self.left_index == self.right_index {
            self.right_index += 1;
        } else {
            self.left_index += 1;
        }
    }
}
impl Iterator for PalindromeIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let str_count = self.input_string.chars().count();

        // if we are within bounds
        if self.left_index < str_count && self.right_index < str_count {
            let result = (self.left_index, self.right_index);
            self.increment();
            return Some(result);
        } else {
            return None;
        }
    }
}

pub fn longest_palindrome_at_indices(input_string: String, indices: (usize, usize)) -> Option<String>{
    let (mut left_i, mut right_i) = indices;
    let byte_string = input_string.as_bytes();

    // return the input_string if length == 1
    if input_string.len() == 1 { return Some(input_string); }

    // initial check for 2-char center, whether the 2-char center is palindromic
    if left_i != right_i && byte_string[left_i] != byte_string[right_i] {
        // in this case, the 2-char center is not palindromic, so just return
        // None.
        return None;
    }

    // while we can decrement/increment one of our two indices
    while left_i > 0 && right_i < byte_string.len()-1 {
        // increment if equal
        if byte_string[left_i] == byte_string[right_i] {
            left_i -= 1;
            right_i += 1;
        } else {
            break;
        }
    }

    if byte_string[left_i] != byte_string[right_i] {
        left_i += 1;
        right_i -= 1;
    }

    let result_string = String::from_utf8(
        byte_string[left_i..right_i+1].to_vec()
    ).ok().unwrap();

    // println!("result_string: {}", result_string);

    return Some(result_string);
}

struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest_palindrome: String = String::from("");

        let pal_iter = PalindromeIterator::new(s.clone());
        for (li, ri) in pal_iter {
            let result = longest_palindrome_at_indices(s.clone(), (li, ri));
            match result {
                Some(palindrome) => {
                    if palindrome.len() > longest_palindrome.len() {
                        longest_palindrome = palindrome;
                    }
                },
                None => continue,
            }
        }
        return longest_palindrome;
    }
}

fn main() {
    let result = Solution::longest_palindrome(String::from("babad"));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_composite_indexes() {

    }
}
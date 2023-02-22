use std::collections::HashMap;
use std::cmp::Reverse;

struct Solution {}

struct RomanNumeralMap {
    map: HashMap<i32, char>
}

impl RomanNumeralMap {
    pub fn new() -> Self {
        let map = HashMap::from([
            (1, 'I'),
            (5, 'V'),
            (10, 'X'),
            (100, 'C'),
            (500, 'D'),
            (1000, 'M'),
        ]);
        return Self {
            map: map
        }
    }

    pub fn integers_ordered(&self) -> Vec<&i32> {
        let mut keys: Vec<&i32> = self.map.keys().collect();
        keys.sort_by(|a, b| b.cmp(a));
        return keys;
    }
}

// 1. Start with the highest roman numeral, and get its integer value.
// 2. Divide the input integer by the roman integer equivalent.
// 3. Save the result of the division (without the remainder).
// 4. Take the result of the division, and add that many roman characters to the
// result string.
// 5. Do modulo division (to get the remainder), and then repeat 1-4 with the remainder
impl Solution {
    pub fn int_to_roman(input_num: i32) -> String {
        let mut roman_string: String = String::new();
        let numeral_map = RomanNumeralMap::new();

        for (num_int, num_rom) in numeral_map.map {
            let multiples = input_num / num_int;
            let rom_numerals = String::from_utf8(vec![num_rom; 10])
        }

        return roman_string;
    }
}

fn main() {
    let result = Solution::int_to_roman(1);
    println!("The result is: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_int_to_roman() {
        let result = Solution::int_to_roman(1);
        assert_eq!(result, "I");
    }
}
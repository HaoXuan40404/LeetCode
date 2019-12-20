pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars:Vec<char> = s.chars().collect();
        let len = chars.len();
        if len < 1 {
            return s;
        }
        let (mut index, mut current_len, mut start, mut end)= (0, 0, 0, 0);
        while index < len {
            let (mut i, mut j) = (index, index);
            let ch = chars[index];
            while i > 0 && chars[i - 1] == ch {
                i = i - 1;
            }
            while j < len - 1 && chars[j + 1] == ch {
                j = j + 1;
            }
            index = j + 1;
            while i > 0 && j < len - 1 && chars[i - 1] == chars[j + 1] {
                i = i -1;
                j = j + 1;
            }
            let max_len = j - i + 1;
            if max_len > current_len {
                current_len = max_len;
                start = i;
                end = j;
            }
            if max_len >= len - 1 {
                break;
            }
        }
        s[start..end + 1].to_string()
    }
}
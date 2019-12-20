pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let (mut start, mut end, mut max) = (0, 0, 0);


        while end < len {
            for index in start..end {
                if chars[end] == chars[index] {
                    start = index + 1;
                    break;
                }
            }
            let now = end - start + 1;
            if now > max {
                max = now;
            }
            end = end + 1;
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

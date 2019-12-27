use std::str::Chars;

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        let mut iters: Vec<Chars> = strs.iter().map(|s| s.chars()).collect();
        if strs.len() < 1 {
            return prefix;
        }
        let mut curr_char: Option<char> = None;
        loop {
//            prefix.push(ch);
            curr_char.take().map(|c| prefix.push(c));
//            curr_char.map(|ch| prefix.push(ch));
            for iter in iters.iter_mut() {
                let mut ch = iter.next();
                if ch.is_none() {
                    return prefix;
                }
                if curr_char.is_none() {
                    curr_char = ch.take();
                }
                else if curr_char.unwrap() != ch.unwrap() {
                    return prefix;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec!["flower","flow","flight"];
        let strs = strs.iter().map(|s| s.to_string()).collect();
        let result = Solution::longest_common_prefix(strs);
        println!("result = {}", result);
    }

}


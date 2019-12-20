pub struct Solution {}

impl Solution {
        pub fn is_palindrome(x: i32) -> bool {
            if x < 0 {
            return false;
        }
            let mut chars: Vec<i32> = Vec::new();
            let mut input = x;
            while input !=0 {
                chars.push(input%10);
                input = input / 10;
            }
            let len = chars.len();
            for i in 0..len/2 {
                if chars[i] != chars[len-i-1] {
                    return false;
                }
            }
            true
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
    fn test_func() {
        let x = 121;
        println!("result = {}", Solution::is_palindrome(x));
    }
}

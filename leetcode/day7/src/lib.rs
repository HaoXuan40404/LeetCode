pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut input = x as i64;
        let mut result = 0;
        let mut digit;
        let upper_bound: i64 = (2 as i64).pow(31) - 1;
        let lower_bound: i64 = (-2 as i64).pow(31);
        while input != 0 {
            digit = input % 10;
            result = result * 10 + digit;
            input = input / 10;
        }
        if result > upper_bound || result < lower_bound {
            return 0;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

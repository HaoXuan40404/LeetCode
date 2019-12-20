pub struct Solution {}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let chars:Vec<char> = str.chars().collect();
        let mut result = 0;
        let mut option = false;
        let mut minus = false;

        let upper_bound: i64 = (2 as i64).pow(31) - 1;
        let lower_bound: i64 = (-2 as i64).pow(31);
        for ch in chars {
            if !option {
                match ch {
                    ' ' => {}
                    '0'..='9' => {
                        option= true;
                        result = result + ch.to_digit(10).unwrap() as i64;
                    }
                    '-' => {
                        option = true;
                        minus = true;
                    }
                    '+' => {
                        option = true;
                    }
                    _ => return 0,
                }
            }
            else {
                match ch {
                    '0'..='9' => {
                        result = result * 10 + ch.to_digit(10).unwrap() as i64;
                        if result > upper_bound {
                            break;
                        }
                    }
                    _ => break,
                }
            }

        }
        result = if minus { -result } else { result };
        if result > upper_bound {
            return upper_bound as i32;
        }
        if result < lower_bound {
            return lower_bound as i32;
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

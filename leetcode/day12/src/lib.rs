pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let table: Vec<(i32, &'static str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut num = num;

        let mut roman_string = String::new();
        for p in table.iter() {
            if num >= p.0 {
                for _ in 0..(num / p.0) {
                    roman_string.push_str(p.1);
                }
                num = num % p.0
            }
        }
        roman_string
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

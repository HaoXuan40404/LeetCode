pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let table: Vec<(char, Vec<char>)> = vec![
            ('0', vec![]),
            ('1', vec![]),
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ];
        if digits.len() < 1 {
            return vec![];
        }
        let mut result: Vec<String> = vec![String::with_capacity(digits.len())];
        for tmp in digits.chars().into_iter() {
            let value = &table[tmp.to_digit(10).unwrap() as usize].1;
            let mut added: Vec<String> = Vec::with_capacity((value.len() - 1) * result.len());
            for append in &mut result {
                for (index, p) in value.iter().enumerate() {
                    // make each single char append to result
                    if index == value.len() - 1 {
                        append.push(*p);
                    }
                    else {
                        let mut new_comb = (*append).clone();
                        new_comb.push(*p);
                        added.push(new_comb);
                    }
                }
            }
            result.append(&mut added);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let result = Solution::letter_combinations("23".to_string());
        println!("result = {:?}", result);
    }
}

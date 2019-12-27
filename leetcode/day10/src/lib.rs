pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut matched = vec![vec![false; p.len() + 1]; 2];
        let (mut now, mut past) = (0, 1);
        for i in (0..s.len() + 1).rev() {
            for j in (0..p.len() + 1).rev() {
                let (text, pattern) = (&s[i..], &p[j..]);
                matched[now][j] = if pattern.is_empty() {
                    text.is_empty()
                } else {
                    let first_match = !text.is_empty()
                        && (text.as_bytes()[0] == pattern.as_bytes()[0]
                            || pattern.as_bytes()[0] == b'.');
                    if pattern.len() >= 2 && pattern.as_bytes()[1] == b'*' {
                        (first_match && matched[past][j]) || matched[now][j + 2]
                    } else {
                        first_match && matched[past][j + 1]
                    }
                };
            }
            std::mem::swap(&mut now, &mut past);
        }
        matched[past][0]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0_usize, (height.len() - 1));
        let mut max_result: i32 = (end - start) as i32 * (min(height[start], height[end]));
        let mut tmp_area;
        while (end - start) > 1 {
            if height[start] < height[end] {
                start += 1;
                if height[start] < height[start - 1] {
                    continue;
                }
            } else {
                end -= 1;
                if height[end] < height[end + 1] {
                    continue;
                }
            }
            tmp_area = (end - start) as i32 * (min(height[start], height[end]));
            if tmp_area > max_result {
                max_result = tmp_area;
            }
        }
        max_result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

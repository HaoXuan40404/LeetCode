use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut previous = nums[0] - 1;
        while i < len - 2 {
            // skip same number
            if nums[i] == previous {
                i += 1;
                continue;
            }
            previous = nums[i];
            let mut vec = Solution::two_sum(&nums[(i + 1)..len], 0 - nums[i]);
            for t in vec.iter() {
                result.push(vec![nums[i], t.0, t.1]);
            }
            i += 1;
        }
        result
    }

    // 2 sum using 2 pointers: nums[0] ->   <- nums[len-1]
    #[inline(always)]
    fn two_sum(nums: &[i32], sum: i32) -> Vec<(i32, i32)> {
        let (mut i, mut j) = (0_usize, nums.len() - 1);
        let mut result = Vec::new();
        while i < j {
            if nums[i] + nums[j] < sum {
                i += 1
            } else if nums[i] + nums[j] > sum {
                j -= 1
            } else {
                result.push((nums[i], nums[j]));
                i = Solution::next_unique(nums, i, true);
                j = Solution::next_unique(nums, j, false);
            }
        }
        result
    }

    // seek next un-repeat number
    #[inline(always)]
    fn next_unique(nums: &[i32], idx: usize, forward: bool) -> usize {
        let curr = nums[idx];
        let mut i = idx;
        while i > 0 && i < nums.len() && nums[i] == curr {
            i = if forward { i + 1 } else { i - 1 }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let target = Solution::three_sum(nums);
        println!("target = {:?}", target);
        let nums = vec![1,2,-2,-1];
        let target = Solution::three_sum(nums);
        println!("target = {:?}", target);
        let nums = vec![0,0,0];
        let target = Solution::three_sum(nums);
        println!("target = {:?}", target);
        let nums = vec![3,0,-2,-1,1,2];
        let target = Solution::three_sum(nums);
        println!("target = {:?}", target);

    }
}

//use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut min_distance: i32 = i32::max_value();
        nums.sort();
        let mut i = 0;
        while i < nums.len() - 2 {
            let sub_min = Solution::two_sum_closest(&nums[(i + 1)..nums.len()], target - nums[i]);
            if sub_min.abs() < min_distance.abs() {
                min_distance = sub_min;
                if min_distance == 0 {
                    break;
                }
            }
            i += 1;
        }
        target + min_distance
    }

//    #[inline(always)]
//    fn two_sum(nums: Vec<i32>, target: i32, jump: usize) -> Vec<i32> {
//        let mut map = HashMap::with_capacity(nums.len());
//        for (index, num) in nums.iter().enumerate() {
//            if index == jump {
//                continue;
//            }
//            match map.get(&(target - num)) {
//                None => {
//                    map.insert(num, index);
//                }
//                Some(sub_index) => return vec![*sub_index as i32, index as i32],
//            }
//        }
//        vec![]
//    }

    #[inline(always)]
    fn two_sum_closest(nums: &[i32], target: i32) -> i32 {
        let (mut i, mut j) = (0 as usize, nums.len() - 1);
        let mut local_min = i32::max_value();
        while i < j {
            let sum = nums[i] + nums[j];
            if sum > target {
                j -= 1;
            } else if sum < target {
                i += 1;
            } else {
                return 0;
            }
            if (sum - target).abs() < local_min.abs() {
                local_min = sum - target
            }
        }
        local_min
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

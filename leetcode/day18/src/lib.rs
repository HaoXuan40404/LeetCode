pub struct Solution {}

use std::collections::BTreeMap;
use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        let mut map: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
        for i in 0..(nums.len()-1) {
            for j in (i+1)..nums.len() {
                map.entry(nums[i] + nums[j]).or_insert(Vec::new()).push((i, j));
            }
        }
        for (&sum, pairs) in map.iter() {
//            if sum > target/2 {
//                break;
//            }
            match map.get(&(target - sum)) {
                None => continue,
                Some(subs) => {
                    for pair in pairs.iter() {
                        for sub in subs.iter() {
                            if sub.0 == pair.0 || sub.0 == pair.1 || sub.1 == pair.0 || sub.1 == pair.1 {
                                continue;
                            }
                            let mut vec = vec![nums[pair.0], nums[pair.1], nums[sub.0], nums[sub.1]];
                            vec.sort();
                            set.insert(vec);
                        }
                    }
                }
            }
        }
        set.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::Solution;

        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2 ], vec![-1, 0, 0, 1]]
        );
    }
}

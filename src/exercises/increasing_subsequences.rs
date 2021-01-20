use std::collections::HashSet;
use std::hash::Hash;

struct Solution {}

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = HashSet::new();
        recur(&nums, i32::min_value(), &mut vec![], &mut results);
        results.into_iter().collect()
    }
}

fn recur(nums: &[i32], last: i32, accum: &mut Vec<i32>, results: &mut HashSet<Vec<i32>>) {
    if nums.len() == 0 {
        if accum.len() > 1 {
            results.insert(accum.clone());
        }
    } else {
        let cur = nums[0];
        if cur >= last {
            accum.push(cur);
            recur(&nums[1..], cur, accum, results);
            accum.pop();
        }
        recur(&nums[1..], last, accum, results);
    }
}

pub trait ToSet<T> {
    fn into_set(self) -> HashSet<T>;
}

impl<I: IntoIterator> ToSet<I::Item> for I
    where I::Item: Eq + Hash {
    fn into_set(self) -> HashSet<I::Item> {
        self.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_subsequences(vec![4, 6, 7, 7]).into_set(), vec![
            vec![4, 6],
            vec![4, 7],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
            vec![4, 7, 7]
        ].into_set());
    }
}
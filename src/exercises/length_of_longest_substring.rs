use std::cmp::max;

pub(crate) struct Solution();

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut mx = 0;
        let chars = s.as_bytes();

        let mut indices = [-1; 256];

        for char in chars {
            let map_key = *char as usize;
            let index = indices[map_key];
            indices[map_key] = j;

            if index >= 0 {
                i = max(index + 1, i);
            }
            mx = max(mx, j - i + 1);
            j += 1;
        }

        mx
    }
}
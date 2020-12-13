use exercises::length_of_longest_substring::Solution;

mod exercises;
fn main() {
    println!("{}", Solution::length_of_longest_substring(String::from("abcabcbb")));
    println!("{}", Solution::length_of_longest_substring(String::from("bbbbb")));
    println!("{}", Solution::length_of_longest_substring(String::from("pwwkew")));
    println!("{}", Solution::length_of_longest_substring(String::from("")));
    println!("{}", Solution::length_of_longest_substring(String::from("tmmzuxt")));
    println!("{}", Solution::length_of_longest_substring(String::from("aabaab!bb")));
}

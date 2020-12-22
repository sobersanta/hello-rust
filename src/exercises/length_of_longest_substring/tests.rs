use super::*;

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    assert_eq!(length_of_longest_substring(String::from("")), 0);
    assert_eq!(length_of_longest_substring(String::from("tmmzuxt")), 5);
    assert_eq!(length_of_longest_substring(String::from("aabaab!bb")), 3);
}

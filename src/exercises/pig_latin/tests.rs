use super::*;

#[test]
fn test_word_to_pig_latin() {
    let str = "first";
    assert_eq!(word_to_pig_latin(str), "irst-fay");
    let str = "apple";
    assert_eq!(word_to_pig_latin(str), "apple-hay");
}
